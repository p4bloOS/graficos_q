// main.rs

extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::event::WindowEvent;
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;



pub fn main() {

    // Inicializamos la biblioteca sdl (contexto_sdl: Sdl )
    let contexto_sdl = sdl2::init().unwrap();

    // Inicializamos subsistema de vídeo
    let subst_video = contexto_sdl.video().unwrap();

    /* Creación de una ventana.
     window(...) retorna un WindowBuilder, y build() retorna una Window */
    let ven = subst_video.window("Mi demo en SDL2", 800, 600)
        .position_centered()
        .build()
        .unwrap();

	/* Creación de un canvas ("lienzo").
	into_canvas() devuelve un constructor de canvas.
	build() devuelve una ventana canvas (canvas<Window>)	*/
    let mut canvas = ven.into_canvas()
    .present_vsync() /*< La pantalla no podrá renderizar más rápido que
                                                    la tasa de refresco */
    .build().unwrap();

    // Creación de un "surtidor" de eventos
	let mut eventos = contexto_sdl.event_pump().unwrap();

    /* Bucle principal del programa.
    Lo creamos con wait_iter porque es más eficiente que consultar constantemente
    con poll_iter (esperar en lugar de consultar) */
	'bucle_de_eventos: for e in eventos.wait_iter() {
	
   	    match e {

   	        // Cortaremos el programa en este caso:
   	        Event::Quit {..} |
   	        Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
   	            println!("Evento de terminación recibido.");
   	            break 'bucle_de_eventos
   	        },

   	        // Solo refrescaremos la imagen cuando se produzca este evento:
   	        Event::Window { win_event: WindowEvent::Exposed, .. } => {
                println!("Evento de ventana expuesta recibido.");
                pintar(&mut canvas);
   	            canvas.present();
   	        },
   	        
            _ => {}
            
   	    }
   	    
   	}
   	
} // Fin de main



/* Con esta función pintamos en el canvas lo que queremos.
   Tras llamar a esta función refrescaremos la ventana con canvas.present()*/
fn pintar(canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) {
    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear(); // < Colorea la pantalla con el color establecido
	
    canvas.set_draw_color(Color::RGB(20,20,20));
    canvas.fill_rect(Rect::new(10,10,200,200)).unwrap();
}
