//! Fichier contenant la logique du jeu tetris (déplacement des tetrminos et tilemaps).

mod map;
mod tetromino;
mod tileset;

pub use map::MAP_DISPLAY_X_OFFSET;

use map::{Map, MAP_WIDTH, MAP_HEIGHT};
use tetromino::{Tetromino, TetrominoKind};
use tileset::{TILES_OUTLINE_THICKNESS, TILES_OUTLINE_COLOR, TILES_WIDTH, TILES_HEIGHT, TILES_COLOR};
use topi::toolkit::time::Timer;

/// Instance gérant l'entièretée de la logique de jeu.
pub struct Game {
    map: Map,
    tetromino: Tetromino,
    next_tetromino: Tetromino,
    falling_preview: Tetromino,
    score: u64,
    level: u64,
    game_over: bool,
    time_to_fall: u64,
    time_before_insertion: u64,
    falling_timer: Timer,
    fast_fall_timer: Timer,
    insertion_timer: Option<Timer>
}

impl Game {
    /// Construit et renvoie une nouvelle instance d'une partie de tetris.
    pub fn new() -> Self {
        let tetro = topi::toolkit::rand::randrange(0usize..7usize);
        let next_tetro = topi::toolkit::rand::randrange(0usize..7usize);
        let tetro = TetrominoKind::try_from(tetro).unwrap();
        let next_tetro = TetrominoKind::try_from(next_tetro).unwrap();

        let time_to_fall = 600;

        Self {
            map: Map::new(),
            tetromino: Tetromino::new(tetro),
            next_tetromino: Tetromino::new(next_tetro),
            falling_preview: Tetromino::new(tetro),
            score: 0u64,
            level: 0u64,
            game_over: false,
            time_to_fall,
            time_before_insertion: 450,
            falling_timer: Timer::new(time_to_fall),
            fast_fall_timer: Timer::new(100),
            insertion_timer: None
        }
    }

    /// Interface sur les entrées du jeu, gère les déplacements du joueur en situation de jeu.
    pub fn handle_inputs(&mut self, input: &topi::InputManager) {
        if input.is_key_pressed(topi::keycode::KEY_UP) && Self::tetromino_can_rotate(&self.tetromino, &self.map) {
            self.tetromino.rotate();
        }

        if input.is_key_pressed(topi::keycode::KEY_LEFT) && Self::tetromino_can_move_left(&self.tetromino, &self.map) {
            self.tetromino.move_left();
        }

        if input.is_key_pressed(topi::keycode::KEY_RIGHT) && Self::tetromino_can_move_right(&self.tetromino, &self.map) {
            self.tetromino.move_right();
        }

        // Fast fall.
        if input.is_key_held(topi::keycode::KEY_DOWN) &&
            Self::tetromino_can_fall(&self.tetromino, &self.map) &&
            self.fast_fall_timer.finished() 
        {
            self.tetromino.fall();
            self.fast_fall_timer.restart();
        }

        // Hard drop.
        if input.is_key_pressed(topi::keycode::KEY_SPACE) {
            while Self::tetromino_can_fall(&self.tetromino, &self.map) {
                self.tetromino.fall();
            }
            self.time_before_insertion = 0;
        }
    }

    /// Interface sur la mise à jour de la logique de jeu courante.
    pub fn update(&mut self) {
        // On détruit les lignes pleines et on garde la trace des lignes détruites.
        let mut nb_line_destroyed = 0;
        while let Some(index) = self.map.get_full_line_index() {
            self.map.destroy_line(index);
            nb_line_destroyed += 1;
        }

        // On attribut le score en fonction des lignes détruites.
        match nb_line_destroyed {
            1 => self.attribute_score(20),
            2 => self.attribute_score(20),
            3 => self.attribute_score(50),
            4 => self.attribute_score(100),
             _ => {}
        }

        // On calcul la prévisualisation du tetromino sur le sol.
        self.falling_preview = self.tetromino;
        while Self::tetromino_can_fall(&self.falling_preview, &self.map) {
            self.falling_preview.fall();
        }

        // Si le tetromino peut tomber alors on le fait tomber.
        if Self::tetromino_can_fall(&self.tetromino, &self.map) {
            if self.falling_timer.finished() {
                self.tetromino.fall();
                self.falling_timer.restart();
            }
        }

        // Sinon, on l'ajoute à la grille définitive, modifie le score et en fait apparaître un
        // nouveau.
        else {
            // On lance le minuteur avant de poser définitivement le tetromino.
            if self.insertion_timer.is_none() {
                self.insertion_timer = Some(Timer::new(self.time_before_insertion));
            }

            // S'il a fini, alors on le pose.
            if let Some(timer) = &self.insertion_timer && timer.finished() {
                // On l'insert dans la carte.
                self.map.insert_tetro(&self.tetromino);

                // On en choisit un nouveau.
                self.pick_next_tetromino();

                // On supprime le minuteur d'insertion.
                self.insertion_timer = None;
                self.time_before_insertion = 450;

                // On regarde si on est en situation de game over.
                self.game_over = !Self::tetromino_can_fall(&self.tetromino, &self.map);

                // On incrémente le score.
                self.attribute_score(10);
            }
        }
    }

    /// Interface sur l'affichage de l'état du jeu.
    pub fn display(&self, renderer: &mut topi::RenderContext) {
        renderer.enable_filling();

        // On affiche le terrain de jeu.
        let cell_w = TILES_WIDTH - 2 * TILES_OUTLINE_THICKNESS;
        let cell_h = TILES_HEIGHT - 2 * TILES_OUTLINE_THICKNESS;
        let outline = TILES_OUTLINE_THICKNESS as i32;
        renderer.set_stroke_weight(TILES_OUTLINE_THICKNESS);

        for x in 0..MAP_WIDTH {
            for y in 0..MAP_HEIGHT {
                let tile = self.map.get(x, y);

                renderer.set_draw_color(&TILES_COLOR[&tile]);
                renderer.set_stroke_color(&TILES_OUTLINE_COLOR[&tile]);

                let screen_x = x as i32 * TILES_WIDTH as i32 + outline + MAP_DISPLAY_X_OFFSET;
                let screen_y = (MAP_HEIGHT - y - 1) as i32 * TILES_HEIGHT as i32 + outline;

                renderer.draw_rect(screen_x, screen_y, cell_w, cell_h);
            }
        }

        // On affiche le tetromino courant et sa prévisualisation sur le sol.
        let tetro_size = self.tetromino.get_size();
        for x in 0..tetro_size {
            for y in 0..tetro_size {
                // Le tetromino courant est le même d'un point de son contenu que sa prévisualisation sur le sol.
                let tile = self.tetromino.get(x, y);
                if tile.is_none() { continue; }

                renderer.set_draw_color(&TILES_COLOR[&tile]);
                renderer.set_stroke_color(&TILES_OUTLINE_COLOR[&tile]);
                renderer.set_stroke_weight(TILES_OUTLINE_THICKNESS);

                // On affiche le courant.
                renderer.enable_filling();

                let dx = x as i32 + self.tetromino.get_pos().get_x();
                let dy = self.tetromino.get_pos().get_y() - y as i32;

                let screen_x = dx * TILES_WIDTH as i32 + outline + MAP_DISPLAY_X_OFFSET;
                let screen_y = (MAP_HEIGHT as i32 - dy - 1) * TILES_HEIGHT as i32 + outline;

                renderer.draw_rect(screen_x, screen_y, cell_w, cell_h);

                // Puis, on affiche sa prévisualisation.
                renderer.disable_filling();

                let dx = x as i32 + self.falling_preview.get_pos().get_x();
                let dy = self.falling_preview.get_pos().get_y() - y as i32;

                let screen_x = dx * TILES_WIDTH as i32 + outline + MAP_DISPLAY_X_OFFSET;
                let screen_y = (MAP_HEIGHT as i32 - dy - 1) * TILES_HEIGHT as i32 + outline;

                renderer.draw_rect(screen_x, screen_y, cell_w, cell_h);
            }
        }

        // On affiche le score et le niveau du joueur.
        renderer.set_font("./JetBrainsMonoNerdFont-Bold.ttf", 30);
        renderer.set_draw_color(&topi::types::color::WHITE);
        renderer.draw_text(&format!("Score: {}", self.score), 5, 10);
        renderer.draw_text(&format!("Level: {}", self.level), 5, 50);

        // On affiche le prochain tetromino à arriver après celui courant.
        renderer.disable_filling();
        renderer.set_stroke_color(&topi::types::color::WHITE);
        let x_rect = 558; let y_rect = 60;
        renderer.draw_rect(x_rect, y_rect, 4*32, 4*32);

        let x_offset = match self.next_tetromino.get_kind() {
            TetrominoKind::I => x_rect + TILES_WIDTH as i32 / 2,
            TetrominoKind::O => x_rect + TILES_WIDTH as i32,
            _ => x_rect + TILES_WIDTH as i32 / 2
        };
        
        let y_offset = match self.next_tetromino.get_kind() {
            TetrominoKind::I => y_rect,
            TetrominoKind::O => y_rect + TILES_HEIGHT as i32,
            _ => y_rect + TILES_HEIGHT as i32 / 2
        };

        renderer.enable_filling();
        let tetro_size = self.next_tetromino.get_size();
        for x in 0..tetro_size {
            for y in 0..tetro_size {
                let tile = self.next_tetromino.get(x, y);
                if tile.is_none() { continue }

                renderer.set_draw_color(&TILES_COLOR[&tile]);
                renderer.set_stroke_color(&TILES_OUTLINE_COLOR[&tile]);
                renderer.set_stroke_weight(TILES_OUTLINE_THICKNESS);

                let dx = x as i32;
                let dy = y as i32;

                let screen_x = dx * TILES_WIDTH as i32 + outline + x_offset;
                let screen_y = dy * TILES_HEIGHT as i32 + outline + y_offset;

                renderer.draw_rect(screen_x, screen_y, cell_w, cell_h);
            }
        }

        // On affiche l'écran de game over
        if self.game_over() {
            renderer.enable_filling();
            renderer.set_stroke_weight(0);
            renderer.set_draw_color(&topi::types::color::BLACK);
            renderer.draw_rect(MAP_DISPLAY_X_OFFSET, 256, 320, 128);

            renderer.set_draw_color(&topi::types::color::RED);
            renderer.set_font("./JetBrainsMonoNerdFont-Bold.ttf", 30);
            renderer.draw_text("GAME OVER", 80 + MAP_DISPLAY_X_OFFSET, 285);

            renderer.set_draw_color(&topi::types::color::WHITE);
            renderer.set_font("./JetBrainsMonoNerdFont-Bold.ttf", 25);
            renderer.draw_text("`space` to replay", 30 + MAP_DISPLAY_X_OFFSET, 325);
        }
    }

    /// Vérifie si le joueur a perdu ou non.
    #[inline]
    pub fn game_over(&self) -> bool {
        self.game_over
    }

    /// Attribut le score en paramètre au joueur et modifie l'état du jeu en fonction du niveau
    /// obtenu par l'incrémentation du score.
    fn attribute_score(&mut self, score: u64) {
        self.score += score;

        if self.score.is_multiple_of(200) {
            self.level += 1;
            
            if self.time_to_fall as i32 - 50 >= 0 {
                self.time_to_fall -= 50;
                self.falling_timer.set_milliseconds_to_wait(self.time_to_fall);
            }
        }
    }

    /// Détermine si le tetromino en paramètre peux-être placé dans la carte en paramètre.
    fn tetromino_can_be_placed(tetromino: &Tetromino, map: &Map) -> bool {
        let size = tetromino.get_size() as i32;
        let pos = tetromino.get_pos();

        for x in 0..size {
            for y in 0..size {
                if tetromino.get(x as usize, y as usize).is_none() { continue; }

                let map_x = pos.get_x() + x;
                let map_y = pos.get_y() - y;

                if map_x < 0
                    || map_x >= MAP_WIDTH as i32
                    || map_y < 0
                    || map_y >= MAP_HEIGHT as i32 {
                        return false
                }

                if map.get(map_x as usize, map_y as usize).is_some() {
                    return false
                }
            }
        }

        true
    }

    /// Vérifie que le tetromino en paramètre peut tomber d'une case dans la carte en paramètre.
    ///
    /// # Arguments
    /// * `tetromino` tetromino dont on veut savoir s'il peut tomber dans la carte en paramètre.
    /// * `map` carte dans laquelle le tetromino essaye de tomber.
    ///
    /// # Return
    /// `true` le tetromino peut tomber, `false` sinon.
    fn tetromino_can_fall(tetromino: &Tetromino, map: &Map) -> bool {
        let mut candidate = *tetromino;
        candidate.fall();
        Self::tetromino_can_be_placed(&candidate, map)
    }

    /// Vérifie que le tetromino en paramètre peut se déplacer sur la droite dans la carte de jeu en
    /// paramètre.
    ///
    /// # Arguments
    /// * `tetromino` tetromino dont on veut savoir s'il peut se déplacer vers la droite.
    /// * `map` carte dans laquelle on veut savoir si le tetromino peut se déplacer.
    ///
    /// # Return
    /// `true` le tetromino peut se déplacer, `false` sinon.
    fn tetromino_can_move_right(tetromino: &Tetromino, map: &Map) -> bool {
        let mut candidate = *tetromino;
        candidate.move_right();
        Self::tetromino_can_be_placed(&candidate, map)
    }

    /// Vérifie que le tetromino en paramètre peut se déplacer sur la gauche dans la carte de jeu en
    /// paramètre.
    ///
    /// # Arguments
    /// * `tetromino` tetromino dont on veut savoir s'il peut se déplacer vers la gauche.
    /// * `map` carte dans laquelle on veut savoir si le tetromino peut se déplacer.
    ///
    /// # Return
    /// `true` le tetromino peut se déplacer, `false` sinon.
    fn tetromino_can_move_left(tetromino: &Tetromino, map: &Map) -> bool {
        let mut candidate = *tetromino;
        candidate.move_left();
        Self::tetromino_can_be_placed(&candidate, map)
    }

    /// Vérifie que le tetromino en paramètre peut effectuer une rotation dans dans la carte de jeu
    /// en paramètre.
    ///
    /// # Arguments
    /// * `tetromino` tetromino dont on veut savoir s'il peut effectuer une rotation.
    /// * `map` carte dans laquelle le tetromino effectuerai la rotation.
    ///
    /// # Return
    /// `true` le tetromino peut effectuer une rotation, `false` sinon.
    fn tetromino_can_rotate(tetromino: &Tetromino, map: &Map) -> bool {
        let mut candidate = *tetromino;
        candidate.rotate();
        Self::tetromino_can_be_placed(&candidate, map)
    }

    /// Place le prochain tetromino comme courant et détermine son prochain.
    fn pick_next_tetromino(&mut self) {
        // On place le prochain comme courant.
        self.tetromino = self.next_tetromino;
        self.falling_preview = self.tetromino;

        // On détermine le suivant
        let next_tetro = topi::toolkit::rand::randrange(0..7usize);
        let next_tetro = TetrominoKind::try_from(next_tetro).unwrap();
        self.next_tetromino = Tetromino::new(next_tetro);
    }
}
