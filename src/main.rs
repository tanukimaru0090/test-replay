use sfml::{window::*,graphics::*,system::*};
use std::fs::*;
use std::io::{Write,BufRead,BufReader};
use regex::Regex;
const MAX_TIME:f32 = 50.0;
#[derive(PartialEq,Clone)]
enum KeyState{
    Left,
    Right,
    Up,
    Down,
    Escape,
    Unknown,
}
fn keystate_tostring(key:&KeyState)->String{
    match key{
        KeyState::Left => {return "Left".to_string();},
        KeyState::Right => {return "Right".to_string();},
        KeyState::Up => {return "Up".to_string();},
        KeyState::Down => {return "Down".to_string();},
        KeyState::Unknown => {return "Unknown".to_string();},
        KeyState::Escape => {return "Escape".to_string();},
    }
}
fn keystate_tonum(key:&KeyState)->i32{
        match key{
            KeyState::Left => {return 1;},
            KeyState::Right => {return 2;},
            KeyState::Up => {return 3;},
            KeyState::Down => {return 4;},
            KeyState::Unknown => {return 5;},
            KeyState::Escape => {return 6;},
    }
}
fn replaydata_tuple_to_replaydata(data:(i32,i32))->ReplayData{
    let mut rep = ReplayData::new();
    let mut key = KeyState::Unknown;
    let mut frame = 0;
    return rep;
}
fn as_replaydata_num(data:String)->(i32,i32){
    let mut index = 0;
    let pattern_regex = Regex::new(r"\b(\d+)\b").unwrap();
    let (mut key,mut frame) = (0,0);
    for cap in pattern_regex.captures_iter(&data){
        index+=1;
        if let Some(number) = cap.get(1){
            if index == 1{
                key = number.as_str().parse::<i32>().unwrap();
            }else if index == 2{
                frame = number.as_str().parse::<i32>().unwrap();
            }
        }
    }
    return (key,frame);
}
#[derive(Clone)]
struct ReplayData{
    key:KeyState,
    frame:i32,
}
impl ReplayData{
    fn new()->ReplayData{return ReplayData{key:KeyState::Unknown,frame:0};}
}
struct Replay{
    replay_data:Vec<ReplayData>,
}
impl Replay{
    fn new()->Replay{return Replay{replay_data:Vec::new()};} 
    fn replay_run(&mut self,player_sprite:&mut Sprite,window:&mut RenderWindow)->Result<(),String>{
        for val in self.replay_data.clone(){
            
        }
        return Ok(());
    }
    fn save_replay_data(replay_path:&str,replay_data:&ReplayData)->Result<(),std::io::Error>{
        let mut file = OpenOptions::new()
            .create(true)
            .append(true) 
            .open(replay_path)?;
        let key_num = keystate_tonum(&replay_data.key);
        let frame_str = replay_data.frame.to_string();
        writeln!(file,"{} {}",key_num,frame_str);
        return Ok(())
    }
    fn load_replay_data(&mut self,replay_path:&str)->Result<Vec<ReplayData>,std::io::Error>{
        let replay_data =Vec::<ReplayData>::new();
        let file = File::open(replay_path)?;
        let reader = BufReader::new(file); 
        let (mut key,mut frame) = (0,0);
        for line in reader.lines(){
                (key,frame) = as_replaydata_num(line.unwrap());
        }
        return Ok(replay_data);
    }
}

fn main()->Result<(),std::io::Error> {
    let mut font = Font::from_file("./assets/Cica-Bold.ttf").expect("フォントの作成に失敗しました");
    let mut clock = Clock::start();
    let mut clock_text = Text::new("",&font,20);  
    let mut replay_data = ReplayData::new();
    let mut window =RenderWindow::new(VideoMode::new(640,480,32),"",Style::DEFAULT,&Default::default());
    let mut frame_text = Text::new("",&font,20);
    let mut player_texture = Texture::new().unwrap();
    match player_texture.load_from_file("assets/rpg-charctors.png", Rect::new(0,0,30,60)){
        Ok(val) => {println!("{:?}",val);},
        Err(err) => {println!("{:?}",err);},
    }
    let mut player_pos = Vector2f::new(300.0,300.0);
    let mut player_sprite = Sprite::new();
    player_sprite.set_texture(&player_texture,false);
    window.set_framerate_limit(60);   
    window.set_vertical_sync_enabled(true);
    frame_text.set_position((0.0,30.0));
    while window.is_open(){
            if let Some(event) =  window.poll_event(){
                match event{
                    Event::Closed => {window.close();},
                    Event::KeyPressed{code,scan,alt,ctrl,shift,system} => {
                        if code == Key::Left{ 
                            replay_data.key = KeyState::Left;
                            player_pos.x-=10.0;
                        }else if code == Key::Right{ 
                            replay_data.key = KeyState::Right;
                            player_pos.x+=10.0;
                        }
                        else if code == Key::Up{ 
                            replay_data.key = KeyState::Up;
                            player_pos.y-=10.0;
                        }
                        else if code == Key::Down{ 
                            replay_data.key = KeyState::Down;
                            player_pos.y+=10.0;
                        }else if code == Key::Escape{ 
                            replay_data.key = KeyState::Escape;
                        }else{
                            replay_data.key = KeyState::Unknown;
                        }

                    },
                    _ => {},

                }
            }
            // エスケープが押されたらウィンドウを閉じる
            if replay_data.key == KeyState::Escape{
                window.close();
            }
            //  毎フレーム、フレームとキー情報を保存
            if replay_data.key != KeyState::Unknown {    
                Replay::save_replay_data("./assets/replay.txt",&replay_data)?;
            }
            

            // Update 
            if clock.elapsed_time().as_seconds() > MAX_TIME{
                println!("目標タイムに到達");
                break;
            }
            player_sprite.set_position(player_pos);
            replay_data.frame+=1;
            clock_text.set_string(&format!("time:{}",clock.elapsed_time().as_seconds()));
            frame_text.set_string(&format!("frame:{}",replay_data.frame.to_string()));
            
            // Draw 
            window.clear(Color::BLACK);
            window.draw(&clock_text);
            window.draw(&frame_text);
            window.draw(&player_sprite);
            window.display();
    }
    return Ok(());
}
