import { Universe, BackgroundElementType, ForegroundElement, ForegroundElementType } from "sokowasm";
import { memory } from "sokowasm/sokowasm_bg";

const CELL_SIZE = 34;
const BACKGROUND_COLOR = '#202020';

document.body.style.backgroundColor = BACKGROUND_COLOR;

const universe = Universe.from_level_const();
console.log(universe, universe.background());

const width = universe.width();
const height = universe.height();
const foreground_size = universe.foreground_size();

const canvas = document.getElementById("sokowasm-canvas");
const summary = document.getElementById("summary");
const youWon = document.getElementById("won");

const img_wall = document.getElementById("img-wall");
const img_goal = document.getElementById("img-goal");
const img_box  = document.getElementById("img-box");
const img_box_ok = document.getElementById("img-box-ok");

//player image credit to: https://opengameart.org/content/alternate-lpc-character-sprites-george
const img_player = [document.getElementById("img-player-up"),
                    document.getElementById("img-player-down"),
                    document.getElementById("img-player-right"),
                    document.getElementById("img-player-left")];

const PLAYER_UP = 0;
const PLAYER_DOWN = 1;
const PLAYER_RIGHT = 2;
const PLAYER_LEFT = 3;

var curr_player_face = PLAYER_DOWN;

canvas.height = (CELL_SIZE) * height+50;
canvas.width = (CELL_SIZE) * width;


const ctx = canvas.getContext('2d');


const renderLoop = () => {
    //universe.tick();

    if( universe.has_won() ) {
        canvas.style.display = "none";
        youWon.style.display = "block";
    }
    else {
        drawBackground();
        drawForeground();
    }

    requestAnimationFrame(renderLoop);
};

const getIndex = (row, column) => {
    return row * width + column;
};

const image_routine = (img, col, row) => {
    ctx.drawImage(img,
        col * (CELL_SIZE),
        row * (CELL_SIZE),
        CELL_SIZE,
        CELL_SIZE
    );
};

const black_routine = (col, row) => {
    ctx.fillStyle = BACKGROUND_COLOR;

    ctx.fillRect(
    col * (CELL_SIZE),
    row * (CELL_SIZE),
    CELL_SIZE,
    CELL_SIZE
    );
}

const drawBackground = () => {    
    for (let row = 0; row < height; row++) {
        for (let col = 0; col < width; col++) {
            black_routine(col, row);
        }
    }

    for (let row = 0; row < height; row++) {
        for (let col = 0; col < width; col++) {

            var img_to_draw = null;
            if (universe.get_background_2D(col,row) === BackgroundElementType.Wall) {
                img_to_draw = img_wall;
            }
            else if (universe.get_background_2D(col,row) === BackgroundElementType.Goal) {
                img_to_draw = img_goal;
            }
            if (img_to_draw) {
                image_routine(img_to_draw, col, row);
            }
        }
    }
};

const drawForeground = () => {
    for (let i = 0 ; i < foreground_size ; i += 1) {
        const elem_type = universe.get_foreground_elem(i).element_type();
        const x = universe.get_foreground_elem(i).x();
        const y = universe.get_foreground_elem(i).y();

        if (elem_type == ForegroundElementType.Player) {
            image_routine(img_player[curr_player_face], x, y);
        }
        else if (elem_type == ForegroundElementType.Crate) {
            var to_draw = img_box;
            if (universe.get_background_2D(x,y) === BackgroundElementType.Goal)
                var to_draw = img_box_ok;
            image_routine(to_draw, x, y);
        }
    }

    summary.innerHTML = universe.number_crates_ok().toString()+"/"+(universe.foreground_size()-1).toString();
};

renderLoop();

document.addEventListener('keyup', 
    function (event) {
        if (event.key == "ArrowUp") {
            universe.move_player(0,-1); 
            curr_player_face = PLAYER_UP;
        }

        if (event.key == "ArrowRight") {
            universe.move_player(1,0); 
            curr_player_face = PLAYER_RIGHT;
        }

        if (event.key == "ArrowDown") {
            universe.move_player(0,1); 
            curr_player_face = PLAYER_DOWN;
        }

        if (event.key == "ArrowLeft") {
            universe.move_player(-1,0); 
            curr_player_face = PLAYER_LEFT;
        }
    }
);