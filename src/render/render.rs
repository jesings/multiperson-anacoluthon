use crate::map::grid;
use crate::gamestate;

pub fn render(gs: ClientGamestate) {
    let canv = gs.sdl.canv.lock().expect("Could not lock canvas for rendering!");
    //get row, col position
    
    //get center of grid from that
    
    //get corners of grid from that
    
    //render tiles between corners
    
    //present canvas
}
