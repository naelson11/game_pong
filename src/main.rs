#![recursion_limit="512"]

// Resto del código del juego de ping pong...
use nalgebra::Point2;
//use mint::Point2 as MintPoint2;

// Importamos las bibliotecas necesarias
use ggez::event;
use ggez::graphics;
use ggez::input::keyboard;
use ggez::{Context, GameResult};
use nalgebra as na;

//nuevo
use std::collections::HashSet;
use ggez::graphics::Text;



// Definimos las constantes para las dimensiones del juego

// Dimensiones de la ventana del juego
const SCREEN_WIDTH: f32 = 800.0;
const SCREEN_HEIGHT: f32 = 600.0;

// Tamaño del marcador
const SCORE_AREA_HEIGHT: f32 = SCREEN_HEIGHT * 0.1;

// Coordenadas para el marcador izquierdo
const LEFT_SCORE_X: f32 = SCREEN_WIDTH * 0.2;
const LEFT_SCORE_Y: f32 = SCORE_AREA_HEIGHT * 0.5;

// Coordenadas para el marcador derecho
const RIGHT_SCORE_X: f32 = SCREEN_WIDTH * 0.8;
const RIGHT_SCORE_Y: f32 = SCORE_AREA_HEIGHT * 0.5;

// Dimensiones de las paletas
const PADDLE_WIDTH: f32 = 20.0;
const PADDLE_HEIGHT: f32 = SCREEN_HEIGHT * 0.9 * 0.2; // 90% del alto de la pantalla y 20% del área de juego

// Dimensiones de la pelota
const BALL_SIZE: f32 = 10.0;

const PADDLE_SPEED: f32 = 11.0;

// Creamos una estructura para representar el estado del juego
struct Pong {
    left_paddle: na::Point2<f32>,
    right_paddle: na::Point2<f32>,
    ball: na::Point2<f32>,
    ball_velocity: na::Vector2<f32>,
    original_ball_velocity: na::Vector2<f32>,
    ball_speed_multiplier: f32, 
        // Agregamos el estado de las teclas presionadas para mover las paletas
    left_paddle_keys: HashSet<keyboard::KeyCode>,
    right_paddle_keys: HashSet<keyboard::KeyCode>,
    
        // Agregamos el contador de puntos para cada jugador
    left_score: i32,
    right_score: i32
}

impl Pong {
    // Función para actualizar el estado del juego
    fn new(ctx: &mut Context) -> GameResult<Pong> {
        // ...

        // Inicializamos la velocidad original y el multiplicador de velocidad
        let left_paddle = na::Point2::new(10.0, 10.0);
        let right_paddle = na::Point2::new(SCREEN_WIDTH - PADDLE_WIDTH - 10.0, 10.0);
        let ball = na::Point2::new(SCREEN_WIDTH / 2.0, SCREEN_HEIGHT / 2.0);
    
        // Inicializamos la velocidad original y el multiplicador de velocidad
        let original_ball_velocity = na::Vector2::new(2.0, 2.0);
        let ball_speed_multiplier = 1.05; // Puedes ajustar este valor según tus preferencias
    
        let pong = Pong {
            left_paddle,
            right_paddle,
            ball,
            ball_velocity: original_ball_velocity, // Usamos la velocidad original aquí
            left_paddle_keys: HashSet::new(),
            right_paddle_keys: HashSet::new(),
            left_score: 0,
            right_score: 0,
            original_ball_velocity,
            ball_speed_multiplier,
        };
    
        Ok(pong)
    }

    fn update(&mut self) {
        // Movimiento de la pelota
        self.ball += self.ball_velocity;
    
        // Comprobar colisión con los bordes superior e inferior
        if self.ball.y <= 0.0 || self.ball.y >= SCREEN_HEIGHT - 10.0 {
            self.ball_velocity.y = -self.ball_velocity.y;
        }
    
        // Comprobar colisión con las paletas izquierda y derecha
        if self.ball.x <= PADDLE_WIDTH
            && self.ball.y >= self.left_paddle.y
            && self.ball.y <= self.left_paddle.y + PADDLE_HEIGHT
        {
            self.ball_velocity.x = -self.ball_velocity.x * self.ball_speed_multiplier;
        }
    
        if self.ball.x >= SCREEN_WIDTH - PADDLE_WIDTH - 10.0
            && self.ball.y >= self.right_paddle.y
            && self.ball.y <= self.right_paddle.y + PADDLE_HEIGHT
        {
            self.ball_velocity.x = -self.ball_velocity.x * self.ball_speed_multiplier;
        }
    
        // Comprobar si la pelota ha salido por la izquierda
        if self.ball.x <= 0.0 {
            // Restablecer la posición de la pelota al centro
            self.ball = na::Point2::new(SCREEN_WIDTH / 2.0, SCREEN_HEIGHT / 2.0);
            // Incrementar el contador de puntos del jugador derecho
            self.right_score += 1;
            // Cambiar la dirección inicial de la pelota
            self.ball_velocity = self.original_ball_velocity;
        }
    
        // Comprobar si la pelota ha salido por la derecha
        if self.ball.x >= SCREEN_WIDTH {
            // Restablecer la posición de la pelota al centro
            self.ball = na::Point2::new(SCREEN_WIDTH / 2.0, SCREEN_HEIGHT / 2.0);
            // Incrementar el contador de puntos del jugador izquierdo
            self.left_score += 1;
            // Cambiar la dirección inicial de la pelota
            self.ball_velocity = self.original_ball_velocity;
        }
    
        // Mover las paletas según las teclas presionadas
        if self.left_paddle_keys.contains(&keyboard::KeyCode::W) {
            self.left_paddle.y -= PADDLE_SPEED;
        }
        if self.left_paddle_keys.contains(&keyboard::KeyCode::S) {
            self.left_paddle.y += PADDLE_SPEED;
        }
        if self.right_paddle_keys.contains(&keyboard::KeyCode::Up) {
            self.right_paddle.y -= PADDLE_SPEED;
        }
        if self.right_paddle_keys.contains(&keyboard::KeyCode::Down) {
            self.right_paddle.y += PADDLE_SPEED;
        }
    
        // Asegurarse de que las paletas no se salgan de los límites de la ventana
        self.left_paddle.y = self.left_paddle.y.max(0.0).min(SCREEN_HEIGHT - PADDLE_HEIGHT);
        self.right_paddle.y = self.right_paddle.y.max(0.0).min(SCREEN_HEIGHT - PADDLE_HEIGHT);
    }
    
    // Función para dibujar los elementos del juego en pantalla
    fn draw(&self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, [0.0, 0.0, 0.0, 1.0].into());

        // Dibujo del marcador izquierdo
        let left_score_text = Text::new(format!("Left: {}", self.left_score));
        graphics::draw(
            ctx,
            &left_score_text,
            graphics::DrawParam::default()
                .dest([LEFT_SCORE_X, LEFT_SCORE_Y])
                .color(graphics::WHITE),
        )?;

        // Dibujo del marcador derecho
        let right_score_text = Text::new(format!("Right: {}", self.right_score));
        graphics::draw(
            ctx,
            &right_score_text,
            graphics::DrawParam::default()
                .dest([RIGHT_SCORE_X, RIGHT_SCORE_Y])
                .color(graphics::WHITE),
        )?;

        // Dibujo de las paletas
        let left_paddle_rect = graphics::Rect::new(self.left_paddle.x, self.left_paddle.y, PADDLE_WIDTH, PADDLE_HEIGHT);
        let left_paddle_mesh = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            left_paddle_rect,
            graphics::WHITE,
        )?;
        graphics::draw(ctx, &left_paddle_mesh, graphics::DrawParam::default())?;

        let right_paddle_rect = graphics::Rect::new(self.right_paddle.x, self.right_paddle.y, PADDLE_WIDTH, PADDLE_HEIGHT);
        let right_paddle_mesh = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            right_paddle_rect,
            graphics::WHITE,
        )?;
        graphics::draw(ctx, &right_paddle_mesh, graphics::DrawParam::default())?;

        // Dibujo de la pelota
        let ball_rect = graphics::Rect::new(self.ball.x, self.ball.y, BALL_SIZE, BALL_SIZE);
        let ball_mesh = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            ball_rect,
            graphics::WHITE,
        )?;
        graphics::draw(ctx, &ball_mesh, graphics::DrawParam::default())?;

        graphics::present(ctx)?;
    
        // Agregar un pequeño retraso para reducir la velocidad de actualización
        std::thread::sleep(std::time::Duration::from_millis(10));
    
        // Indicar que el dibujo se realizó exitosamente
        Ok(())
    }
    
    fn handle_key_down_event(&mut self, keycode: keyboard::KeyCode, is_pressed: bool) {
        match keycode {
            // Teclas para controlar la paleta izquierda
            keyboard::KeyCode::W => {
                if is_pressed {
                    self.left_paddle_keys.insert(keycode);
                } else {
                    self.left_paddle_keys.remove(&keycode);
                }
            }
            keyboard::KeyCode::S => {
                if is_pressed {
                    self.left_paddle_keys.insert(keycode);
                } else {
                    self.left_paddle_keys.remove(&keycode);
                }
            }

            // Teclas para controlar la paleta derecha
            keyboard::KeyCode::Up => {
                if is_pressed {
                    self.right_paddle_keys.insert(keycode);
                } else {
                    self.right_paddle_keys.remove(&keycode);
                }
            }
            keyboard::KeyCode::Down => {
                if is_pressed {
                    self.right_paddle_keys.insert(keycode);
                } else {
                    self.right_paddle_keys.remove(&keycode);
                }
            }

            _ => (),
        }
    }
}

// Implementamos el trait event::EventHandler para manejar los eventos del juego
// Implementamos el trait event::EventHandler para manejar los eventos del juego


// Implementamos el trait event::EventHandler para manejar los eventos del juego
impl event::EventHandler for Pong {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        if self.left_paddle_keys.contains(&keyboard::KeyCode::W) {
            self.left_paddle.y -= PADDLE_SPEED;
        }
        if self.left_paddle_keys.contains(&keyboard::KeyCode::S) {
            self.left_paddle.y += PADDLE_SPEED;
        }
        if self.right_paddle_keys.contains(&keyboard::KeyCode::Up) {
            self.right_paddle.y -= PADDLE_SPEED;
        }
        if self.right_paddle_keys.contains(&keyboard::KeyCode::Down) {
            self.right_paddle.y += PADDLE_SPEED;
        }

        // Evitar que las paletas se salgan de los límites de la ventana
        self.left_paddle.y = self.left_paddle.y.max(0.0).min(SCREEN_HEIGHT - PADDLE_HEIGHT);
        self.right_paddle.y = self.right_paddle.y.max(0.0).min(SCREEN_HEIGHT - PADDLE_HEIGHT);
        self.update();
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, [0.0, 0.0, 0.0, 1.0].into());

        let ball_rect = graphics::Rect::new(self.ball.x, self.ball.y, 10.0, 10.0);
        let ball_mesh = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            ball_rect,
            graphics::WHITE,
        )?;
        graphics::draw(ctx, &ball_mesh, graphics::DrawParam::default())?;

        let left_paddle_rect = graphics::Rect::new(self.left_paddle.x, self.left_paddle.y, PADDLE_WIDTH, PADDLE_HEIGHT);
        let left_paddle_mesh = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            left_paddle_rect,
            graphics::WHITE,
        )?;
        graphics::draw(ctx, &left_paddle_mesh, graphics::DrawParam::default())?;

        let right_paddle_rect = graphics::Rect::new(self.right_paddle.x, self.right_paddle.y, PADDLE_WIDTH, PADDLE_HEIGHT);
        let right_paddle_mesh = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            right_paddle_rect,
            graphics::WHITE,
        )?;
        graphics::draw(ctx, &right_paddle_mesh, graphics::DrawParam::default())?;

        graphics::present(ctx)?;
        
        // Agregamos un pequeño retraso para reducir la velocidad de actualización
        std::thread::sleep(std::time::Duration::from_millis(10));

        Ok(())
    }

    fn key_down_event(&mut self, _ctx: &mut Context, keycode: keyboard::KeyCode, _keymods: keyboard::KeyMods, _repeat: bool) {
        // Control de las paletas con las teclas de flecha y 'W' y 'S'
        match keycode {
            keyboard::KeyCode::Up => self.right_paddle.y -= PADDLE_SPEED,
            keyboard::KeyCode::Down => self.right_paddle.y += PADDLE_SPEED,
            keyboard::KeyCode::W => self.left_paddle.y -= PADDLE_SPEED,
            keyboard::KeyCode::S => self.left_paddle.y += PADDLE_SPEED,
            _ => (),
        }
    }
}

fn main() -> GameResult {
    // Creamos una instancia del contexto de ggez
    let cb = ggez::ContextBuilder::new("pong", "ggez").window_setup(ggez::conf::WindowSetup::default().title("Pong"));
    let (ctx, event_loop) = &mut cb.build()?;

    // Inicializamos el estado del juego
    let mut initial_state = Pong {
        left_paddle: na::Point2::new(10.0, 10.0),
        right_paddle: na::Point2::new(SCREEN_WIDTH - PADDLE_WIDTH - 10.0, 10.0),
        ball: na::Point2::new(SCREEN_WIDTH / 2.0, SCREEN_HEIGHT / 2.0),
        ball_velocity: na::Vector2::new(2.0, 2.0),
        left_paddle_keys: HashSet::new(),
        right_paddle_keys: HashSet::new(),
        original_ball_velocity: na::Vector2::new(2.0, 2.0),
        ball_speed_multiplier: 1.5, // Puedes ajustar este valor según tus preferencias

             // Inicializamos los contadores de puntos
        left_score: 0,
        right_score: 0,
    };

    // Ejecutamos el bucle principal del juego
    event::run(ctx, event_loop, &mut initial_state)
}
