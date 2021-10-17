//Comando para executar o exemplo "cargo run --release --features "winit glium" --example file_navigator"

extern crate time;

use crate::cenario::Cenario;
use crate::sessao::{Sessao, TipoSessao};
use crate::support;
use crate::teste::{Teste, Tipos};
use crate::lol::{LoLU, User, LoLState, Settings, LoLSession};
use crate::db_structures::{MatchData, MatchStats, MatchTimeline, PlotableValues, LoLData};
use crate::lol_structs::{ChampionMasteryDTO, ParticipantDto, ObjectivesDto};
use crate::empatica;

use conrod;
use conrod::backend::glium::glium;
use conrod::backend::glium::glium::Surface;
use conrod::backend::winit;
use conrod::color;
use find_folder;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;
use std::path::PathBuf;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::hash_map::RandomState;

//use device_query::{DeviceQuery, DeviceState, Keycode};
use std::time::Duration;
//use std::time::Instant;
use std::sync::mpsc;

pub struct Valores {
    teste: Teste,
    sessoes: Vec<Sessao>,
    sessoes_novas: Vec<Sessao>,

    //nome: String,
    num_vals: String,
    //executavel: String,
    executavel_caminho: Option<PathBuf>,
    executavel_nome: String,
    opcao: Option<usize>,
    opcao_export: Option<usize>,
    opcao_saidas: Vec<bool>,
    opcao_cenario: Option<usize>,
    variaveis: Vec<String>,
    num_cens: String,
    cenarios: Vec<String>,
    valores: Vec<String>,
    pagina: i64,
    selecionado: u32,
    ajuda: i64,
    pagina_anterior: i64,

    load: bool,
    lol_user: Option<LoLU>,
    username: Option<String>,
    user: User,
    championMastery: Vec<ChampionMasteryDTO>,
    list_selected: HashSet<usize, RandomState>,
    lane_sprites:  Vec<conrod::image::Id>,
    lanes_selected: HashSet<usize, RandomState>,
    channel_receiver: Option<std::sync::mpsc::Receiver<Result<(),usize>>>,
    session_receiver: Option<std::sync::mpsc::Receiver<LoLSession>>,
    timer: std::time::SystemTime,

    search_data: SearchData,
    found_matches: Vec<usize>,
    match_selected: Option<String>,
    timeline_view: bool,

    found_match: Option<MatchData>,
    found_match_stats: Option<MatchStats>,
    found_match_timeline: Option<MatchTimeline>,

    changing_location: bool,
    exe_folder: Option<std::path::PathBuf>,
    exe_new_location: Option<String>,

    plot_view_data: PlotData,

    empatica_files_location: Option<String>,

    display_data: DisplayData,
}

pub struct Gui {}

impl Gui {
    pub fn new() -> Gui {
        Gui {}
    }

    pub fn run(&self) {
        const WIDTH: u32 = 1920;
        const HEIGHT: u32 = 1080;

        // Build the window.
        let mut events_loop = glium::glutin::EventsLoop::new();
        let window = glium::glutin::WindowBuilder::new()
            .with_title("Focus")
            .with_maximized(true)
            .with_dimensions(WIDTH, HEIGHT)
            .with_fullscreen(None)
            .with_visibility(true);
        let context = glium::glutin::ContextBuilder::new().with_vsync(true);
        //            .with_multisampling(4);

        let display = glium::Display::new(window, context, &events_loop).unwrap();

        // Construct our `Ui`.
        let mut ui = conrod::UiBuilder::new([WIDTH as f64, HEIGHT as f64]).build();

        // A unique identifier for each widget.
        //    let ids = Ids::new(ui.widget_id_generator());
        let ids = &mut Ids::new(ui.widget_id_generator());

        // Add a `Font` to the `Ui`'s `font::Map` from file.
        let assets = find_folder::Search::KidsThenParents(3, 5)
            .for_folder("assets")
            .unwrap();
        let font_path = assets.join("fonts/NotoSans/NotoSans-Regular.ttf");
        ui.fonts.insert_from_file(font_path).unwrap();

        // A type used for converting `conrod::render::Primitives` into `Command`s that can be used
        // for drawing to the glium `Surface`.
        let mut renderer = conrod::backend::glium::Renderer::new(&display).unwrap();

        // The image map describing each of our widget->image mappings (in our case, none).
        let mut image_map = conrod::image::Map::<glium::texture::Texture2d>::new();

        

        // Some starting text to edit.
        let mut valores = Valores {
            teste: Teste {
                nome: "Name".to_string(),
                executavel: "Executable".to_string(),
                variaveis: Vec::new(),
                tipo: Tipos::Vazio,
                sessao: 0,
            },
            sessoes: Vec::new(),
            sessoes_novas: Vec::new(),

            //nome: "Nome".to_string(),
            opcao: None,
            opcao_export: None,
            opcao_saidas: Vec::new(),
            opcao_cenario: None,
            valores: Vec::new(),
            variaveis: Vec::new(),
            num_vals: "0".to_string(),
            num_cens: "0".to_string(),
            cenarios: Vec::new(),
            executavel_nome: "Executable".to_string(),
            executavel_caminho: None,
            pagina: 0,
            selecionado: 9999,
            ajuda: 0,
            pagina_anterior: 0,

            load: false,
            lol_user: None,
            username: Some("Intsuyou".to_string()),
            user: User::new(),
            championMastery: Vec::new(),
            list_selected: HashSet::new(),
            lane_sprites: Vec::new(),
            lanes_selected: HashSet::new(),
            channel_receiver: None,
            session_receiver: None,
            timer: std::time::SystemTime::now(),

            search_data: SearchData::new(),
            found_matches: Vec::new(),
            match_selected: None,
            timeline_view: false,

            found_match: None,
            found_match_stats: None,
            found_match_timeline: None,

            changing_location: false,
            exe_folder: None,
            exe_new_location: None,

            plot_view_data: PlotData::new(),

            empatica_files_location: None,

            display_data: DisplayData{
                champions_image_map: HashMap::new(),
                itens_image_map: HashMap::new(),
                display,
                image_map,
            },
        };

        //valores.champions_image_map = LoLU::get_champions_images(&mut image_map, &display);
        valores.lane_sprites = LoLU::get_lanes_images(&mut valores.display_data.image_map, &valores.display_data.display);

        ids.lanes.resize(5, &mut ui.widget_id_generator());
        ids.laneBG.resize(5, &mut ui.widget_id_generator());

        ids.match_result.resize(2, &mut ui.widget_id_generator());
        ids.match_team_color.resize(2, &mut ui.widget_id_generator());
        ids.first_blood.resize(2, &mut ui.widget_id_generator());
        ids.team_kills.resize(2, &mut ui.widget_id_generator());
        ids.first_tower.resize(2, &mut ui.widget_id_generator());
        ids.tower_kills.resize(2, &mut ui.widget_id_generator());
        ids.first_inhibitor.resize(2, &mut ui.widget_id_generator());
        ids.inhibitor_kills.resize(2, &mut ui.widget_id_generator());
        ids.first_dragon.resize(2, &mut ui.widget_id_generator());
        ids.dragon_kills.resize(2, &mut ui.widget_id_generator());
        ids.first_baron.resize(2, &mut ui.widget_id_generator());
        ids.baron_kills.resize(2, &mut ui.widget_id_generator());
        ids.first_herald.resize(2, &mut ui.widget_id_generator());
        ids.herald_kills.resize(2, &mut ui.widget_id_generator());

        ids.player_image.resize(10, &mut ui.widget_id_generator());
        ids.player_level.resize(10, &mut ui.widget_id_generator());
        ids.player_name.resize(10, &mut ui.widget_id_generator());
        ids.player_kills.resize(10, &mut ui.widget_id_generator());
        ids.player_deaths.resize(10, &mut ui.widget_id_generator());
        ids.player_assists.resize(10, &mut ui.widget_id_generator());
        ids.player_gold.resize(10, &mut ui.widget_id_generator());
        ids.player_itens.resize(70, &mut ui.widget_id_generator());

        //bluetooth::connect();

        // Poll events from the window.
        let mut event_loop = support::EventLoop::new();
        'main: loop {
            // Handle all events.
            for event in event_loop.next(&mut events_loop) {
                // Use the `winit` backend feature to convert the winit event to a conrod one.
                if let Some(event) = winit::convert_event(event.clone(), &valores.display_data.display) {
                    ui.handle_event(event);
                    event_loop.needs_update();
                }

                match event {
                    glium::glutin::Event::WindowEvent { event, .. } => match event {
                        // Break from the loop upon `Escape`.
                        glium::glutin::WindowEvent::Closed
                        | glium::glutin::WindowEvent::KeyboardInput {
                            input:
                                glium::glutin::KeyboardInput {
                                    virtual_keycode: Some(glium::glutin::VirtualKeyCode::Escape),
                                    ..
                                },
                            ..
                        } => break 'main,
                        _ => (),
                    },
                    _ => (),
                }
            }

            // Instnatiate all widgets in the GUI.
            if valores.pagina == 3 {
                set_widgets_3(ui.set_widgets(), ids, &mut valores);
            } else if valores.pagina == 2 {
                set_widgets_2(ui.set_widgets(), ids, &mut valores);
            } else if valores.pagina == 1 {
                set_widgets_1(ui.set_widgets(), ids, &mut valores);
            } else if valores.pagina == 4 {
                set_widgets_4(ui.set_widgets(), ids, &mut valores);
            } else if valores.pagina == 5 {
                set_widgets_5(ui.set_widgets(), ids, &mut valores);
            } else if valores.pagina == 6 {
                set_widgets_6(ui.set_widgets(), ids, &mut valores);
            } else if valores.pagina == 7 {
                set_widgets_7(ui.set_widgets(), ids, &mut valores);
            } else if valores.pagina == 8 {
                set_widgets_8(ui.set_widgets(), ids, &mut valores);
            }else if valores.pagina == 9{
                set_widgets_9(ui.set_widgets(), ids, &mut valores);
            }else if valores.pagina == 10{
                set_widgets_10(ui.set_widgets(), ids, &mut valores);
            }else if valores.pagina == 11{
                set_widgets_11(ui.set_widgets(), ids, &mut valores);
            }else if valores.pagina == 12{
                set_widgets_12(ui.set_widgets(), ids, &mut valores);
            }else if valores.pagina == 13{
                set_widgets_13(ui.set_widgets(), ids, &mut valores);
            }else if valores.pagina == 14{
                set_widgets_14(ui.set_widgets(), ids, &mut valores);
            }else if valores.pagina == 20{
                set_widgets_20(ui.set_widgets(), ids, &mut valores);
            }else if valores.pagina == 0{
                set_widgets_0(ui.set_widgets(), ids, &mut valores)
            }

            // Render the `Ui` and then display it on the screen.
            if let Some(primitives) = ui.draw_if_changed() {
                renderer.fill(&valores.display_data.display, primitives, &valores.display_data.image_map);
                let mut target = valores.display_data.display.draw();
                target.clear_color(0.0, 0.0, 0.0, 1.0);
                renderer.draw(&valores.display_data.display, &mut target, &valores.display_data.image_map).unwrap();
                target.finish().unwrap();
            }
        }

        //valores.sessoes = valores.teste.ler_sessoes();
        let mut arquivo = valores.teste.escrever();

        //valores.sessoes.append(&mut valores.sessoes_novas);

        for cada_sessao in valores.sessoes.iter() {
            cada_sessao.escrever(&mut arquivo);
        }

        // Página para entrar na versão para jogos gerais ou LoL.
        fn set_widgets_0(ref mut ui: conrod::UiCell, ids: &mut Ids, valores: &mut Valores) {
            use conrod::{color, widget, Colorable, Labelable, Positionable, Sizeable, Widget};

            widget::Canvas::new()
                .flow_down(&[
                    (
                        ids.header,
                        widget::Canvas::new()
                            .color(color::DARK_CHARCOAL)
                            .pad_bottom(1.0),
                    ),
                    (
                        ids.body,
                        widget::Canvas::new().flow_right(&[
                            (
                                ids.left_column,
                                widget::Canvas::new().color(color::DARK_CHARCOAL),
                            ),
                            (
                                ids.right_column,
                                widget::Canvas::new().color(color::DARK_CHARCOAL),
                            ),
                        ]),
                    ),
                ])
                .set(ids.master, ui);

            widget::Text::new("What version would you like to use?")
                .color(color::WHITE)
                .font_size(48)
                .middle_of(ids.header)
                .set(ids.titulo, ui);

            for _click in widget::Button::new()
                .color(color::WHITE)
                .label("Regular")
                .label_color(color::BLACK)
                .w_h(100.0, 60.0)
                .middle_of(ids.left_column)
                .set(ids.abrir, ui)
            {
                valores.pagina = 1;
            }

            for _click in widget::Button::new()
                .color(color::WHITE)
                .label("LoL")
                .label_color(color::BLACK)
                .w_h(100.0, 60.0)
                .middle_of(ids.right_column)
                .set(ids.criar, ui)
            {
                if valores.load && valores.username.is_some() {
                    load_lol_user(ui, ids, &mut valores.lol_user, valores.username.as_ref().unwrap());
                    /*valores.lol_user = Some(LoLU::load(valores.username.as_ref().unwrap()));
                    //LoLU::get_matches_id(&valores.lol_user.as_ref().unwrap().user.accountId);

                    let len = valores.lol_user.as_ref().unwrap().matches.len();
                    ids.match_card_outline.resize(len, &mut ui.widget_id_generator());
                    ids.match_card_hero.resize(len, &mut ui.widget_id_generator());
                    ids.match_card_lane.resize(len, &mut ui.widget_id_generator());
                    ids.match_card_more.resize(len, &mut ui.widget_id_generator());
                    ids.match_card_start.resize(len, &mut ui.widget_id_generator());
                    ids.match_card_start_time.resize(len, &mut ui.widget_id_generator());
                    ids.match_card_duration.resize(len, &mut ui.widget_id_generator());
                    ids.match_card_season.resize(len, &mut ui.widget_id_generator());
                    ids.match_card_result.resize(len, &mut ui.widget_id_generator());

                    let len = valores.lol_user.as_ref().unwrap().champion_stats.len();
                    ids.champion_card_outline.resize(len, &mut ui.widget_id_generator());
                    ids.champion_card_hero.resize(len, &mut ui.widget_id_generator());
                    ids.champion_card_more.resize(len, &mut ui.widget_id_generator());
                    ids.champion_card_matches.resize(len, &mut ui.widget_id_generator());
                    ids.champion_card_wins.resize(len, &mut ui.widget_id_generator());
                    ids.champion_card_kpa.resize(len, &mut ui.widget_id_generator());
                    ids.champion_card_kp.resize(len, &mut ui.widget_id_generator());
                    ids.champion_card_gp.resize(len, &mut ui.widget_id_generator());
                    ids.champion_card_matches_text.resize(len, &mut ui.widget_id_generator());
                    ids.champion_card_wins_text.resize(len, &mut ui.widget_id_generator());
                    ids.champion_card_kpa_text.resize(len, &mut ui.widget_id_generator());
                    ids.champion_card_kp_text.resize(len, &mut ui.widget_id_generator());
                    ids.champion_card_gp_text.resize(len, &mut ui.widget_id_generator());

                    let len = valores.lol_user.as_ref().unwrap().lane_stats.len();
                    ids.lane_card_outline.resize(len, &mut ui.widget_id_generator());
                    ids.lane_card_lane.resize(len, &mut ui.widget_id_generator());
                    ids.lane_card_more.resize(len, &mut ui.widget_id_generator());
                    ids.lane_card_matches.resize(len, &mut ui.widget_id_generator());
                    ids.lane_card_wins.resize(len, &mut ui.widget_id_generator());
                    ids.lane_card_kpa.resize(len, &mut ui.widget_id_generator());
                    ids.lane_card_kp.resize(len, &mut ui.widget_id_generator());
                    ids.lane_card_gp.resize(len, &mut ui.widget_id_generator());
                    ids.lane_card_matches_text.resize(len, &mut ui.widget_id_generator());
                    ids.lane_card_wins_text.resize(len, &mut ui.widget_id_generator());
                    ids.lane_card_kpa_text.resize(len, &mut ui.widget_id_generator());
                    ids.lane_card_kp_text.resize(len, &mut ui.widget_id_generator());
                    ids.lane_card_gp_text.resize(len, &mut ui.widget_id_generator());*/

                    valores.pagina = 10;
                    
                } else{
                    valores.username = Some(format!("Intsuyou"));
                    valores.pagina = 9;
                    /*if !valores.images_loaded{
                        valores.champions_image_map = LoLU::get_champions_images(image_map, display);
                        valores.images_loaded = true;
                    }*/
                }
            }

            if !valores.load{
                for _click in widget::Button::new()
                    .color(color::WHITE)
                    .label("Load?")
                    .label_color(color::BLACK)
                    .w_h(100.0, 60.0)
                    .mid_bottom_with_margin_on(ids.right_column, 50.0)
                    .set(ids.load, ui)
                {
                    valores.load = true;
                }
            }
            

            if valores.load {
                let directory = find_folder::Search::KidsThenParents(3, 5)
                .for_folder("Data/LoL/Users")
                .unwrap();

                for event in widget::FileNavigator::with_extension(&directory, &["luser"])
                    .color(conrod::color::LIGHT_BLUE)
                    .font_size(16)
                    .w_h(300.0, 100.0)
                    .mid_bottom_with_margin_on(ids.right_column, 100.0)
                    //.show_hidden_files(true)  // Use this to show hidden files
                    .set(ids.file_navigator, ui)
                {
                    if let conrod::widget::file_navigator::Event::ChangeSelection(arquivos) = event {
                        if arquivos.len() == 1 {
                            valores.username = Some(format!("{}", arquivos[0].file_stem().unwrap().to_str().unwrap()));
                            //valores.nome = format!("{}", arquivos[0].file_stem().unwrap().to_str().unwrap());
                        }else if arquivos.len() == 0{
                            valores.username = None;
                        }
                    }
                }

                for _click in widget::Button::new()
                    .color(color::WHITE)
                    .label("Cancel")
                    .label_color(color::BLACK)
                    .w_h(100.0, 60.0)
                    .mid_bottom_with_margin_on(ids.right_column, 10.0)
                    .set(ids.cancel, ui)
                {
                    valores.load = false;
                }
                
            }
            


            for _click in widget::Button::new()
                .color(color::WHITE)
                .label("?")
                .label_color(color::BLACK)
                .w_h(30.0, 30.0)
                .bottom_left_with_margins_on(ids.left_column, 10.0, 10.0)
                .set(ids.ajuda, ui)
            {
                valores.pagina_anterior = 0;
                valores.pagina = 7;
            }
        }

        // Pagina de criacao ou abertura de teste.
        fn set_widgets_1(ref mut ui: conrod::UiCell, ids: &mut Ids, valores: &mut Valores) {
            use conrod::{color, widget, Colorable, Labelable, Positionable, Sizeable, Widget};

            widget::Canvas::new()
                .flow_down(&[
                    (
                        ids.header,
                        widget::Canvas::new()
                            .color(color::DARK_CHARCOAL)
                            .pad_bottom(1.0),
                    ),
                    (
                        ids.body,
                        widget::Canvas::new().flow_right(&[
                            (
                                ids.left_column,
                                widget::Canvas::new().color(color::DARK_CHARCOAL),
                            ),
                            (
                                ids.right_column,
                                widget::Canvas::new().color(color::DARK_CHARCOAL),
                            ),
                        ]),
                    ),
                ])
                .set(ids.master, ui);

            widget::Text::new("Open test or create a new one?")
                .color(color::WHITE)
                .font_size(48)
                .middle_of(ids.header)
                .set(ids.titulo, ui);

            for _click in widget::Button::new()
                .color(color::WHITE)
                .label("Open")
                .label_color(color::BLACK)
                .w_h(60.0, 60.0)
                .middle_of(ids.left_column)
                .set(ids.abrir, ui)
            {
                valores.pagina = 4;
            }

            for _click in widget::Button::new()
                .color(color::WHITE)
                .label("Create")
                .label_color(color::BLACK)
                .w_h(60.0, 60.0)
                .middle_of(ids.right_column)
                .set(ids.criar, ui)
            {
                valores.pagina = 3;
            }

            for _click in widget::Button::new()
                .color(color::WHITE)
                .label("?")
                .label_color(color::BLACK)
                .w_h(30.0, 30.0)
                .bottom_left_with_margins_on(ids.left_column, 10.0, 10.0)
                .set(ids.ajuda, ui)
            {
                valores.pagina_anterior = 1;
                valores.pagina = 7;
            }
        }

        // Pagina de abertura de executavel.
        fn set_widgets_2(ref mut ui: conrod::UiCell, ids: &mut Ids, valores: &mut Valores) {
            use conrod::{widget, Colorable, Labelable, Positionable, Sizeable, Widget};

            //let ui = &mut ui.set_widgets();
            let directory = find_folder::Search::KidsThenParents(3, 5)
                .for_folder("Jogos")
                .unwrap();

            widget::Canvas::new()
                .color(conrod::color::DARK_CHARCOAL)
                .set(ids.canvas, ui);

            for event in widget::FileNavigator::all(&directory)
                .color(conrod::color::LIGHT_BLUE)
                .font_size(16)
                .wh_of(ids.canvas)
                .middle_of(ids.canvas)
                //.show_hidden_files(true)  // Use this to show hidden files
                .set(ids.navegador_executavel, ui)
            {
                if let conrod::widget::file_navigator::Event::ChangeSelection(arquivos) = event {
                    if arquivos.len() == 1 {
                        if arquivos[0].is_file() {
                            let mut pai = arquivos[0].parent().unwrap();
                            while pai.file_name().unwrap() != Path::new("Jogos") {
                                pai = pai.parent().unwrap();
                            }

                            valores.teste.executavel = format!(
                                "{}",
                                arquivos[0].strip_prefix(pai).unwrap().to_str().unwrap()
                            );
                            valores.executavel_nome =
                                format!("{}", arquivos[0].file_name().unwrap().to_str().unwrap());
                            valores.executavel_caminho = Some(arquivos[0].clone());
                        }
                    }
                }
            }

            for _click in widget::Button::new()
                .bottom_right_with_margins_on(ids.canvas, 50.0, 50.0)
                .color(conrod::color::WHITE)
                .label("Ready")
                .label_color(conrod::color::BLACK)
                .w_h(100.0, 40.0)
                .set(ids.pronto, ui)
            {
                valores.pagina = 3;
            }

            for _click in widget::Button::new()
                .color(conrod::color::WHITE)
                .label("?")
                .label_color(conrod::color::BLACK)
                .w_h(30.0, 30.0)
                .bottom_left_with_margins_on(ids.canvas, 10.0, 10.0)
                .set(ids.ajuda, ui)
            {
                valores.pagina_anterior = 2;
                valores.pagina = 7;
            }

            for _click in widget::Button::new()
                .bottom_left_with_margins_on(ids.canvas, 10.0, 50.0)
                .color(conrod::color::WHITE)
                .label("Return")
                .label_color(conrod::color::BLACK)
                .w_h(100.0, 40.0)
                .set(ids.voltar, ui)
            {
                valores.pagina = 3;
            }
        }

        // Pagina de criacao de teste.
        fn set_widgets_3(ref mut ui: conrod::UiCell, ids: &mut Ids, valores: &mut Valores) {
            use conrod::{
                color, widget, Borderable, Colorable, Labelable, Positionable, Sizeable, Widget,
            };

            widget::Canvas::new()
                .flow_down(&[
                    (
                        ids.header,
                        widget::Canvas::new()
                            .length(240.0)
                            .color(color::DARK_CHARCOAL)
                            .pad_bottom(20.0)
                            .flow_right(&[
                                (
                                    ids.left_column,
                                    widget::Canvas::new()
                                        .color(color::DARK_CHARCOAL)
                                        .length(500.0),
                                ),
                                (
                                    ids.right_column,
                                    widget::Canvas::new()
                                        .color(color::DARK_CHARCOAL)
                                        .scroll_kids_vertically()
                                        .length(ui.win_w - 500.0),
                                ),
                            ]),
                    ),
                    (
                        ids.entrada,
                        widget::Canvas::new().color(color::WHITE).scroll_kids(),
                    ),
                    (
                        ids.footer,
                        widget::Canvas::new().length(60.0).color(color::BLACK),
                    ),
                ])
                .set(ids.master, ui);

            widget::Text::new("Create test")
                .top_left_with_margins_on(ids.left_column, 20.0, 20.0)
                .color(color::WHITE)
                .font_size(48)
                .set(ids.titulo, ui);

            for event in widget::TextBox::new(&valores.teste.nome)
                .top_left_with_margins_on(ids.left_column, 80.0, 20.0)
                .font_size(20)
                .w_h(320.0, 40.0)
                .border(1.0)
                .border_color(color::BLACK)
                .color(color::WHITE)
                .set(ids.nome_edit, ui)
            {
                match event {
                    //widget::text_box::Event::Enter => println!("TextBox: {:?}", valores.nome),
                    //widget::text_box::Event::Update(string) => valores.nome = string.to_string(),
                    widget::text_box::Event::Enter => (),
                    widget::text_box::Event::Update(string) => {
                        valores.teste.nome = string.to_string()
                    }
                }
            }

            for event in widget::TextBox::new(&valores.executavel_nome)
                .top_left_with_margins_on(ids.left_column, 130.0, 20.0)
                .font_size(20)
                .w_h(320.0, 40.0)
                .border(1.0)
                .border_color(color::BLACK)
                .color(color::WHITE)
                .set(ids.executavel_edit, ui)
            {
                match event {
                    //widget::text_box::Event::Enter => println!("TextBox: {:?}", valores.nome),
                    //widget::text_box::Event::Update(string) => valores.nome = string.to_string(),
                    widget::text_box::Event::Enter => (),
                    widget::text_box::Event::Update(string) => {
                        valores.teste.executavel = string.to_string()
                    }
                }
            }

            for _click in widget::Button::new()
                .top_left_with_margins_on(ids.left_column, 130.0, 360.0)
                .color(color::WHITE)
                .label("Search...")
                .label_color(color::BLACK)
                .w_h(100.0, 40.0)
                .set(ids.procurar, ui)
            {
                valores.pagina = 2;
            }

            for _click in widget::Button::new()
                .color(color::WHITE)
                .label("?")
                .label_color(color::BLACK)
                .w_h(30.0, 30.0)
                .bottom_left_with_margins_on(ids.footer, 10.0, 10.0)
                .set(ids.ajuda, ui)
            {
                valores.pagina_anterior = 3;
                valores.pagina = 7;
            }

            for _click in widget::Button::new()
                .top_left_with_margins_on(ids.footer, 10.0, 50.0)
                .color(conrod::color::WHITE)
                .label("Return")
                .label_color(conrod::color::BLACK)
                .w_h(100.0, 40.0)
                .set(ids.voltar, ui)
            {
                valores.pagina = 1;
            }

            if valores.executavel_caminho.is_some() {
                let mut caminho_arquivo = valores.executavel_caminho.clone().unwrap();
                caminho_arquivo.set_file_name("README");
                let retorno_arquivo = File::open(caminho_arquivo);

                //println!("{:?}", retorno_arquivo);
                if retorno_arquivo.is_ok() {
                    let arquivo = retorno_arquivo.unwrap();
                    let mut buf_leitor = BufReader::new(arquivo);
                    let mut readme = String::new();

                    buf_leitor.read_to_string(&mut readme).unwrap();

                    widget::Text::new(&readme)
                        .top_left_with_margins_on(ids.right_column, 10.0, 20.0)
                        .color(color::WHITE)
                        .left_justify()
                        .line_spacing(10.0)
                        .padded_w_of(ids.right_column, 20.0)
                        .wrap_by_word()
                        .set(ids.readme, ui);
                }
            }

            let tipos = vec![
                "Empty".to_string(),
                "Light".to_string(),
                "Scenarios".to_string(),
            ];

            for selecionado in widget::DropDownList::new(&tipos, valores.opcao)
                .w_h(150.0, 40.0)
                .top_left_with_margins_on(ids.left_column, 180.0, 20.0)
                .max_visible_items(3)
                .color(color::WHITE)
                .border(1.0)
                .border_color(color::BLACK)
                .label("Type")
                .label_color(color::BLACK)
                .scrollbar_on_top()
                .set(ids.tipos, ui)
            {
                valores.opcao = Some(selecionado);
                //println!("Escolhido: {:?}", valores.opcao);
                if selecionado == 0 {
                    valores.teste.tipo = Tipos::Vazio;
                } else if selecionado == 1 {
                    valores.teste.tipo = Tipos::Leve;
                } else if selecionado == 2 {
                    valores.teste.tipo = Tipos::Cenarios(Vec::new());
                }
            }

            let mut num_cenarios: usize = 0;
            let mut num_variaveis: usize = 0;

            if valores.opcao != None && valores.opcao != Some(0) {
                widget::Text::new("Variables:")
                    .top_left_with_margins_on(ids.left_column, 190.0, 180.0)
                    .color(color::WHITE)
                    .font_size(20)
                    .set(ids.texto_variaveis, ui);

                for event in widget::TextBox::new(&valores.num_vals)
                    .top_left_with_margins_on(ids.left_column, 180.0, 280.0)
                    .font_size(20)
                    .w_h(30.0, 40.0)
                    .border(1.0)
                    .border_color(color::BLACK)
                    .color(color::WHITE)
                    .set(ids.num_vals, ui)
                {
                    match event {
                        //widget::text_box::Event::Enter => println!("Num vals: {:?}", valores.num_vals),
                        widget::text_box::Event::Enter => (),
                        widget::text_box::Event::Update(string) => valores.num_vals = string,
                    }
                }

                num_variaveis = match valores.num_vals.trim_right().parse::<usize>() {
                    Ok(valor) => valor,
                    Err(_) => 0 as usize,
                };

                valores
                    .variaveis
                    .resize(num_variaveis, "Variable".to_string());

                ids.vars
                    .resize(num_variaveis, &mut ui.widget_id_generator());

                for x in 0..num_variaveis {
                    for event in widget::TextBox::new(&valores.variaveis[x])
                        .top_left_with_margins_on(ids.entrada, 60.0 + 40.0 * (x as f64), 20.0)
                        .font_size(20)
                        .w_h(160.0, 40.0)
                        .border(1.0)
                        .border_color(color::BLACK)
                        .color(color::WHITE)
                        .set(ids.vars[x], ui)
                    {
                        match event {
                            //widget::text_box::Event::Enter => println!("TextBox {}: {:?}", x, valores.variaveis[x]),
                            widget::text_box::Event::Enter => (),
                            widget::text_box::Event::Update(string) => {
                                valores.variaveis[x] = string.to_string()
                            }
                        }
                    }
                }

                if valores.opcao == Some(2) {
                    widget::Text::new("Scenarios:")
                        .top_left_with_margins_on(ids.left_column, 190.0, 320.0)
                        .color(color::WHITE)
                        .font_size(20)
                        .set(ids.texto_cenario, ui);

                    for event in widget::TextBox::new(&valores.num_cens)
                        .top_left_with_margins_on(ids.left_column, 180.0, 420.0)
                        .font_size(20)
                        .w_h(30.0, 40.0)
                        .border(1.0)
                        .border_color(color::BLACK)
                        .color(color::WHITE)
                        .set(ids.num_cens, ui)
                    {
                        match event {
                            //widget::text_box::Event::Enter => println!("Num cens: {:?}", valores.num_cens),
                            widget::text_box::Event::Enter => (),
                            widget::text_box::Event::Update(string) => valores.num_cens = string,
                        }
                    }

                    num_cenarios = match valores.num_cens.trim_right().parse::<usize>() {
                        Ok(valor) => valor,
                        Err(_) => 0 as usize,
                    };

                    valores
                        .cenarios
                        .resize(num_cenarios, "Scenario".to_string());

                    ids.cens.resize(num_cenarios, &mut ui.widget_id_generator());

                    for x in 0..num_cenarios {
                        for event in widget::TextBox::new(&valores.cenarios[x])
                            .top_left_with_margins_on(ids.entrada, 20.0, 180.0 + 100.0 * (x as f64))
                            .font_size(20)
                            .w_h(100.0, 40.0)
                            .border(1.0)
                            .border_color(color::BLACK)
                            .color(color::WHITE)
                            .set(ids.cens[x], ui)
                        {
                            match event {
                                //widget::text_box::Event::Enter => println!("TextBox {}: {:?}", x, valores.cenarios[x]),
                                widget::text_box::Event::Enter => (),
                                widget::text_box::Event::Update(string) => {
                                    valores.cenarios[x] = string.to_string()
                                }
                            }
                        }
                    }

                    let total_variaveis = num_cenarios * num_variaveis;

                    valores.valores.resize(total_variaveis, "0".to_string());
                    ids.vals
                        .resize(total_variaveis, &mut ui.widget_id_generator());

                    for x in 0..total_variaveis {
                        for event in widget::TextBox::new(&valores.valores[x])
                            .top_left_with_margins_on(
                                ids.entrada,
                                60.0 + 40.0 * ((x / num_cenarios) as f64),
                                180.0 + 100.0 * ((x % num_cenarios) as f64),
                            )
                            .font_size(20)
                            .w_h(100.0, 40.0)
                            .border(1.0)
                            .border_color(color::BLACK)
                            .color(color::WHITE)
                            .set(ids.vals[x], ui)
                        {
                            match event {
                                //widget::text_box::Event::Enter => println!("TextBox {}: {:?}", x, valores.valores[x]),
                                widget::text_box::Event::Enter => (),
                                widget::text_box::Event::Update(string) => {
                                    valores.valores[x] = string.to_string()
                                }
                            }
                        }
                    }
                }
            }

            for _click in widget::Button::new()
                .top_right_with_margins_on(ids.footer, 10.0, 10.0)
                .color(color::WHITE)
                .label("Finish")
                .label_color(color::BLACK)
                .w_h(100.0, 40.0)
                .set(ids.ok, ui)
            {
                for x in 0..num_variaveis {
                    valores.teste.variaveis.push(valores.variaveis[x].clone());
                }

                for x in 0..num_cenarios {
                    if let Tipos::Cenarios(ref mut cenarios) = valores.teste.tipo {
                        let (valores1, valores2) = valores.valores.split_at(x * num_variaveis);
                        cenarios.push(
                            Cenario::new(&valores.cenarios[x])
                                .fill(&valores.teste.variaveis, valores2),
                        );
                    }
                }
                let mut arquivo = valores.teste.escrever();
                valores.pagina = 5;
            }
        }

        // Pagina de selecao de arquivo de teste.
        fn set_widgets_4(ref mut ui: conrod::UiCell, ids: &mut Ids, valores: &mut Valores) {
            use conrod::{widget, Colorable, Labelable, Positionable, Sizeable, Widget};

            //let ui = &mut ui.set_widgets();
            let directory = find_folder::Search::KidsThenParents(3, 5)
                .for_folder("Testes")
                .unwrap();

            widget::Canvas::new()
                .color(conrod::color::DARK_CHARCOAL)
                .set(ids.canvas, ui);

            for event in widget::FileNavigator::with_extension(&directory, &["kans"])
                .color(conrod::color::LIGHT_BLUE)
                .font_size(16)
                .wh_of(ids.canvas)
                .middle_of(ids.canvas)
                //.show_hidden_files(true)  // Use this to show hidden files
                .set(ids.file_navigator, ui)
            {
                if let conrod::widget::file_navigator::Event::ChangeSelection(arquivos) = event {
                    if arquivos.len() == 1 {
                        valores.teste = Teste::carregar(
                            arquivos[0].file_stem().unwrap().to_str().unwrap(),
                            arquivos[0].to_str().unwrap(),
                        );
                        //valores.nome =format!("{}", arquivos[0].file_stem().unwrap().to_str().unwrap());
                    }
                }
            }

            for _click in widget::Button::new()
                .color(conrod::color::WHITE)
                .label("?")
                .label_color(conrod::color::BLACK)
                .w_h(30.0, 30.0)
                .bottom_left_with_margins_on(ids.canvas, 10.0, 10.0)
                .set(ids.ajuda, ui)
            {
                valores.pagina_anterior = 4;
                valores.pagina = 7;
            }

            for _click in widget::Button::new()
                .bottom_left_with_margins_on(ids.canvas, 10.0, 50.0)
                .color(conrod::color::WHITE)
                .label("Return")
                .label_color(conrod::color::BLACK)
                .w_h(100.0, 40.0)
                .set(ids.voltar, ui)
            {
                valores.pagina = 1;
            }

            for _click in widget::Button::new()
                .bottom_right_with_margins_on(ids.canvas, 50.0, 50.0)
                .color(conrod::color::WHITE)
                .label("Ready")
                .label_color(conrod::color::BLACK)
                .w_h(100.0, 40.0)
                .set(ids.pronto, ui)
            {
                match valores.teste.tipo {
                    Tipos::Vazio => valores.opcao = Some(0),
                    Tipos::Leve => {
                        valores.opcao = Some(1);
                        valores.variaveis.clear();
                        valores.num_vals = valores.teste.variaveis.len().to_string();
                        for i in 0..valores.teste.variaveis.len() {
                            valores.variaveis.push(valores.teste.variaveis[i].clone());
                        }
                    }
                    Tipos::Cenarios(ref cenarios) => {
                        valores.opcao = Some(2);
                        valores.num_vals = valores.teste.variaveis.len().to_string();
                        valores.num_cens = cenarios.len().to_string();
                    }
                };
                valores.sessoes = valores.teste.ler_sessoes();
                valores.fill();
                valores.pagina = 5;
            }
        }

        // Pagina de teste aberto.
        fn set_widgets_5(ref mut ui: conrod::UiCell, ids: &mut Ids, valores: &mut Valores) {
            use conrod::{
                color, widget, Borderable, Colorable, Labelable, Positionable, Sizeable, Widget,
            };

            widget::Canvas::new()
                .flow_down(&[
                    (
                        ids.header,
                        widget::Canvas::new()
                            .length(200.0)
                            .color(color::DARK_CHARCOAL)
                            .pad_bottom(20.0),
                    ),
                    (
                        ids.entrada,
                        widget::Canvas::new().color(color::WHITE).scroll_kids(),
                    ),
                    (
                        ids.footer,
                        widget::Canvas::new().length(60.0).color(color::BLACK),
                    ),
                ])
                .set(ids.master, ui);

            for _click in widget::Button::new()
                .top_right_with_margins_on(ids.footer, 10.0, 10.0)
                .color(color::WHITE)
                .label("Run")
                .label_color(color::BLACK)
                .w_h(100.0, 40.0)
                .set(ids.executar, ui)
            {
                valores.pagina = 8;
            }

            for _click in widget::Button::new()
                .top_right_with_margins_on(ids.footer, 10.0, 120.0)
                .color(color::WHITE)
                .label("Export")
                .label_color(color::BLACK)
                .w_h(100.0, 40.0)
                .set(ids.exportar, ui)
            {
                valores.pagina = 6;
            }

            for _click in widget::Button::new()
                .top_right_with_margins_on(ids.footer, 10.0, 240.0)
                .color(color::WHITE)
                .label("Process")
                .label_color(color::BLACK)
                .w_h(100.0, 40.0)
                .set(ids.processar, ui)
            {
                for cada_sessao in valores.sessoes.iter_mut() {
                    cada_sessao.processar_video();
                }
                valores.pagina = 5;
            }

            for _click in widget::Button::new()
                .top_right_with_margins_on(ids.footer, 10.0, 360.0)
                .color(color::WHITE)
                .label("Analyze")
                .label_color(color::BLACK)
                .w_h(100.0, 40.0)
                .set(ids.analise, ui)
            {
                //valores.pagina = 5;
            }

            for _click in widget::Button::new()
                .color(color::WHITE)
                .label("?")
                .label_color(color::BLACK)
                .w_h(30.0, 30.0)
                .bottom_left_with_margins_on(ids.footer, 10.0, 10.0)
                .set(ids.ajuda, ui)
            {
                valores.pagina_anterior = 5;
                valores.pagina = 7;
            }

            /*for _click in widget::Button::new()
                .top_right_with_margins_on(ids.footer, 10.0, 360.0)
                .color(color::WHITE)
                .label("Editar")
                .label_color(color::BLACK)
                .w_h(100.0, 40.0)
                .set(ids.editar, ui)
            {
                    valores.pagina = 3;
            }*/

            widget::Text::new("Test Name: ")
                .top_left_with_margins_on(ids.header, 20.0, 20.0)
                .color(color::WHITE)
                .font_size(48)
                .set(ids.title_aux, ui);

            widget::Text::new(valores.teste.nome.as_str())
                .top_left_with_margins_on(ids.header, 20.0, 370.0)
                .color(color::RED)
                .font_size(48)
                .set(ids.title, ui);

            widget::Text::new("Executable Name: ")
                .top_left_with_margins_on(ids.header, 80.0, 20.0)
                .color(color::WHITE)
                .font_size(48)
                .set(ids.executavel_aux, ui);

            widget::Text::new(valores.teste.executavel.as_str())
                .top_left_with_margins_on(ids.header, 80.0, 490.0)
                .color(color::RED)
                .font_size(48)
                .set(ids.executavel, ui);

            widget::Text::new("Type of test: ")
                .top_left_with_margins_on(ids.header, 140.0, 20.0)
                .color(color::WHITE)
                .font_size(24)
                .set(ids.tipo_aux, ui);

            let tipos = vec![
                "Empty".to_string(),
                "Light".to_string(),
                "Scenarios".to_string(),
            ];

            widget::Text::new(tipos[valores.opcao.unwrap()].as_str())
                .top_left_with_margins_on(ids.header, 140.0, 180.0)
                .color(color::RED)
                .font_size(24)
                .set(ids.tipo, ui);

            if valores.opcao != None && valores.opcao != Some(0) {
                let num_variaveis = match valores.num_vals.trim_right().parse::<usize>() {
                    Ok(valor) => valor,
                    Err(_) => 0 as usize,
                };

                ids.vars_nome
                    .resize(num_variaveis, &mut ui.widget_id_generator());

                for x in 0..num_variaveis {
                    widget::Text::new(valores.variaveis[x].as_str())
                        .top_left_with_margins_on(ids.entrada, 60.0 + 40.0 * (x as f64), 20.0)
                        .font_size(20)
                        .w_h(160.0, 40.0)
                        .color(color::BLACK)
                        .set(ids.vars_nome[x], ui);
                }

                if valores.opcao == Some(1) {
                    valores.valores.resize(num_variaveis, "0".to_string());
                    ids.vars
                        .resize(num_variaveis, &mut ui.widget_id_generator());

                    for x in 0..num_variaveis {
                        for event in widget::TextBox::new(&valores.valores[x])
                            .top_left_with_margins_on(ids.entrada, 55.0 + 40.0 * (x as f64), 120.0)
                            .font_size(20)
                            .w_h(160.0, 40.0)
                            .border(1.0)
                            .border_color(color::BLACK)
                            .color(color::WHITE)
                            .center_justify()
                            .set(ids.vars[x], ui)
                        {
                            match event {
                                //widget::text_box::Event::Enter => println!("TextBox {}: {:?}", x, valores.variaveis[x]),
                                widget::text_box::Event::Enter => (),
                                widget::text_box::Event::Update(string) => {
                                    valores.valores[x] = string.to_string()
                                }
                            }
                        }
                    }
                } else {
                    let mut color_saidas = color::WHITE;

                    if let None = valores.opcao_cenario {
                        color_saidas = color::DARK_GREEN;
                    }

                    for _click in widget::Button::new()
                        .top_left_with_margins_on(ids.entrada, 20.0, 20.0)
                        .color(color_saidas)
                        .label("Random")
                        .label_color(color::BLACK)
                        .w_h(160.0, 40.0)
                        .set(ids.aleatorio, ui)
                    {
                        valores.opcao_cenario = None;
                    }

                    let num_cenarios = match valores.num_cens.trim_right().parse::<usize>() {
                        Ok(valor) => valor,
                        Err(_) => 0 as usize,
                    };

                    ids.cens_nome
                        .resize(num_cenarios, &mut ui.widget_id_generator());

                    for x in 0..num_cenarios {
                        color_saidas = color::WHITE;
                        if let Some(escolha) = valores.opcao_cenario {
                            if escolha == x {
                                color_saidas = color::DARK_GREEN;
                            }
                        }
                        for _click in widget::Button::new()
                            .top_left_with_margins_on(ids.entrada, 20.0, 180.0 + 100.0 * (x as f64))
                            .color(color_saidas)
                            .label(valores.cenarios[x].as_str())
                            .label_color(color::BLACK)
                            .w_h(100.0, 40.0)
                            .set(ids.cens_nome[x], ui)
                        {
                            valores.opcao_cenario = Some(x);
                        }
                    }

                    /*
                        widget::Text::new(valores.cenarios[x].as_str())
                            .top_left_with_margins_on(ids.entrada, 20.0, 180.0 + 100.0*(x as f64) )
                            .font_size(20)
                            .w_h(100.0, 40.0)
                            .color(color::BLACK)
                            .set(ids.cens_nome[x], ui);
                    }
                    */
                    let total_variaveis = num_cenarios * num_variaveis;

                    ids.vals_nome
                        .resize(total_variaveis, &mut ui.widget_id_generator());

                    for x in 0..total_variaveis {
                        widget::Text::new(valores.valores[x].as_str())
                            .top_left_with_margins_on(
                                ids.entrada,
                                60.0 + 40.0 * ((x / num_cenarios) as f64),
                                180.0 + 100.0 * ((x % num_cenarios) as f64),
                            )
                            .font_size(20)
                            .w_h(100.0, 40.0)
                            .color(color::BLACK)
                            .center_justify()
                            .set(ids.vals_nome[x], ui);
                    }
                }
            }

            for _click in widget::Button::new()
                .top_left_with_margins_on(ids.footer, 10.0, 50.0)
                .color(conrod::color::WHITE)
                .label("Return")
                .label_color(conrod::color::BLACK)
                .w_h(100.0, 40.0)
                .set(ids.voltar, ui)
            {
                //valores.zerar();
                valores.pagina = 1;
            }
        }

        //Pagina de exportar teste.
        fn set_widgets_6(ref mut ui: conrod::UiCell, ids: &mut Ids, valores: &mut Valores) {
            use conrod::{
                color, widget, Borderable, Colorable, Labelable, Positionable, Sizeable, Widget,
            };

            widget::Canvas::new()
                .flow_down(&[
                    (
                        ids.header,
                        widget::Canvas::new()
                            .length(130.0)
                            .color(color::DARK_CHARCOAL)
                            .pad_bottom(20.0),
                    ),
                    (
                        ids.entrada,
                        widget::Canvas::new()
                            .color(color::WHITE)
                            .scroll_kids()
                            .flow_right(&[
                                (
                                    ids.left_column,
                                    widget::Canvas::new()
                                        .color(color::WHITE)
                                        .scroll_kids()
                                        .length(150.0),
                                ),
                                (
                                    ids.right_column,
                                    widget::Canvas::new().color(color::WHITE).scroll_kids(),
                                ),
                            ]),
                    ),
                    (
                        ids.footer,
                        widget::Canvas::new().length(60.0).color(color::BLACK),
                    ),
                ])
                .set(ids.master, ui);

            let mut selecionado = 0;
            let mut labels: Vec<String> = Vec::new();
            let mut labels_temp: Vec<String> = Vec::new();
            let mut selecionados = 0;

            if valores.opcao_export == Some(1) {
                //println!("{:?}", valores.sessoes);
                if valores.sessoes.len() > 0 {
                    if valores.selecionado != 9999 {
                        selecionado = valores.selecionado as usize;
                    }
                    labels_temp = valores.sessoes[selecionado].dados.nomes_variaveis();

                    if labels_temp.len() != valores.opcao_saidas.len() {
                        valores.opcao_saidas.clear();
                        for x in 0..labels_temp.len() {
                            valores.opcao_saidas.push(false);
                        }
                    }

                    ids.selecionar_saidas.resize(
                        valores.opcao_saidas.len() as usize,
                        &mut ui.widget_id_generator(),
                    );
                    ids.rectangle_saida.resize(
                        valores.opcao_saidas.len() as usize,
                        &mut ui.widget_id_generator(),
                    );

                    for x in 0..labels_temp.len() {
                        let mut color_saidas = color::WHITE;
                        if valores.opcao_saidas[x] {
                            color_saidas = color::DARK_GREEN;
                        }
                        for _click in widget::Button::new()
                            .top_left_with_margins_on(
                                ids.left_column,
                                50.0 + 40.0 * ((x) as f64),
                                10.0,
                            )
                            .color(color_saidas)
                            .label(&labels_temp[x])
                            .label_color(color::BLACK)
                            .w_h(100.0, 40.0)
                            .set(ids.selecionar_saidas[x as usize], ui)
                        {
                            valores.opcao_saidas[x] = !valores.opcao_saidas[x];
                        }
                    }
                }

                for x in 0..valores.opcao_saidas.len() {
                    if valores.opcao_saidas[x] {
                        selecionados += 1;
                        labels.push(labels_temp[x].clone());
                    }
                }

                let numero_var = format!("{}/8", selecionados);
                widget::Text::new(&numero_var)
                    .top_left_with_margins_on(ids.left_column, 10.0, 10.0)
                    .font_size(20)
                    .color(color::BLACK)
                    .set(ids.numero, ui);
            }

            for _click in widget::Button::new()
                .top_right_with_margins_on(ids.footer, 10.0, 10.0)
                .color(color::WHITE)
                .label("Export")
                .label_color(color::BLACK)
                .w_h(100.0, 40.0)
                .set(ids.exportar, ui)
            {
                valores.pagina = 5;

                if valores.opcao_export == Some(0) {
                    // Exportar como CSV

                    if valores.sessoes.len() > 0 {
                        let mut arquivo = valores.teste.escrever();

                        for cada_sessao in valores.sessoes.iter_mut() {
                            cada_sessao.processar_video();
                            cada_sessao.escrever(&mut arquivo);
                        }
                        valores.teste.exportar(&valores.sessoes);
                    }
                } else if valores.opcao_export == Some(1) {
                    //Exportar como gráfico

                    let mut arquivo = valores.teste.escrever();

                    for cada_sessao in valores.sessoes.iter_mut() {
                        cada_sessao.processar_video();
                        cada_sessao.escrever(&mut arquivo);
                    }

                    valores.sessoes[valores.selecionado as usize]
                        .exportar_grafico(selecionados, labels.clone());
                } else if valores.opcao_export == Some(2) {
                    //Exportar como vídeo

                    valores.sessoes[valores.selecionado as usize].exportar_video();

                    let mut arquivo = valores.teste.escrever();

                    for cada_sessao in valores.sessoes.iter_mut() {
                        cada_sessao.processar_video();
                        cada_sessao.escrever(&mut arquivo);
                    }
                }
            }

            for _click in widget::Button::new()
                .top_left_with_margins_on(ids.footer, 10.0, 50.0)
                .color(color::WHITE)
                .label("Return")
                .label_color(color::BLACK)
                .w_h(100.0, 40.0)
                .set(ids.voltar, ui)
            {
                valores.pagina = 5;
            }

            for _click in widget::Button::new()
                .color(color::WHITE)
                .label("?")
                .label_color(color::BLACK)
                .w_h(30.0, 30.0)
                .bottom_left_with_margins_on(ids.footer, 10.0, 10.0)
                .set(ids.ajuda, ui)
            {
                valores.pagina_anterior = 6;
                valores.pagina = 7;
            }

            widget::Text::new("Export as:")
                .top_left_with_margins_on(ids.header, 10.0, 10.0)
                .font_size(48)
                .color(color::WHITE)
                .set(ids.titulo, ui);

            let tipos = vec!["CSV".to_string(), "Graph".to_string(), "Video".to_string()];

            for selecionado in widget::DropDownList::new(&tipos, valores.opcao_export)
                .w_h(150.0, 40.0)
                .top_left_with_margins_on(ids.header, 80.0, 20.0)
                .max_visible_items(3)
                .color(color::WHITE)
                .border(1.0)
                .border_color(color::BLACK)
                .label("Options")
                .label_color(color::BLACK)
                .scrollbar_on_top()
                .set(ids.tipos, ui)
            {
                valores.opcao_export = Some(selecionado);
            }

            if valores.opcao_export == Some(1) || valores.opcao_export == Some(2) {
                widget::Text::new("Session")
                    .top_left_with_margins_on(ids.right_column, 10.0, 120.0)
                    .font_size(20)
                    .w_h(100.0, 40.0)
                    .color(color::BLACK)
                    .set(ids.xis, ui);

                widget::Text::new("Start")
                    .top_left_with_margins_on(ids.right_column, 10.0, 120.0 + 110.0)
                    .font_size(20)
                    .w_h(200.0, 40.0)
                    .color(color::BLACK)
                    .set(ids.data, ui);

                widget::Text::new("Finish")
                    .top_left_with_margins_on(ids.right_column, 10.0, 120.0 + 320.0)
                    .font_size(20)
                    .w_h(200.0, 40.0)
                    .color(color::BLACK)
                    .set(ids.ypslom, ui);

                ids.sessoes.resize(
                    (valores.sessoes.len() * 3) as usize,
                    &mut ui.widget_id_generator(),
                );
                ids.rectangle.resize(
                    (valores.sessoes.len()) as usize,
                    &mut ui.widget_id_generator(),
                );
                ids.selecionar.resize(
                    (valores.sessoes.len()) as usize,
                    &mut ui.widget_id_generator(),
                );

                for x in 0..(valores.sessoes.len() as u32) {
                    if x == valores.selecionado {
                        widget::Rectangle::fill([640.0, 40.0])
                            .top_left_with_margins_on(
                                ids.right_column,
                                50.0 + 40.0 * ((x) as f64),
                                10.0,
                            )
                            .color(color::DARK_GREEN)
                            .set(ids.rectangle[x as usize], ui);
                    } else {
                        widget::Rectangle::outline([640.0, 40.0])
                            .top_left_with_margins_on(
                                ids.right_column,
                                50.0 + 40.0 * ((x) as f64),
                                10.0,
                            )
                            .color(color::DARK_GREEN)
                            .set(ids.rectangle[x as usize], ui);
                    }

                    for _click in widget::Button::new()
                        .top_left_with_margins_on(
                            ids.right_column,
                            50.0 + 40.0 * ((x) as f64),
                            10.0,
                        )
                        .color(color::WHITE)
                        .label("Escolher")
                        .label_color(color::BLACK)
                        .w_h(100.0, 40.0)
                        .set(ids.selecionar[x as usize], ui)
                    {
                        valores.selecionado = x;
                        //println!("{:?}", valores.selecionado);
                    }

                    let mut diferencial = 0.0;

                    for y in 0..3 {
                        let texto: String;
                        if y == 0 {
                            texto = format!("{}", valores.sessoes[x as usize].sessao_atual);
                        } else if y == 1 {
                            texto = get_data(valores.sessoes[x as usize].data_inicio);
                        } else {
                            texto = get_data(valores.sessoes[x as usize].data_conclusao);
                        }
                        if y == 2 {
                            diferencial = 100.0;
                        }
                        widget::Text::new(texto.as_str())
                            .top_left_with_margins_on(
                                ids.right_column,
                                60.0 + 40.0 * ((x) as f64),
                                120.0 + diferencial + 100.0 * ((y) as f64),
                            )
                            .font_size(20)
                            .w_h(200.0, 40.0)
                            .color(color::BLACK)
                            .set(ids.sessoes[(x * 3 + y) as usize], ui);
                    }
                }
            }
        }

        //Pagina de mostra de ajuda
        fn set_widgets_7(ref mut ui: conrod::UiCell, ids: &mut Ids, valores: &mut Valores) {
            use conrod::{color, widget, Colorable, Labelable, Positionable, Sizeable, Widget};

            let nome_arquivo = format!("./Ajuda/ajuda_{}.txt", valores.pagina_anterior);
            let arquivo = File::open(nome_arquivo).expect("Error when opening the test file.");
            let mut buf_leitor = BufReader::new(arquivo);
            let mut ajuda = String::new();

            buf_leitor.read_to_string(&mut ajuda).unwrap();

            widget::Canvas::new()
                .flow_down(&[
                    (
                        ids.header,
                        widget::Canvas::new()
                            .color(color::DARK_CHARCOAL)
                            .pad_bottom(1.0)
                            .length(100.0),
                    ),
                    (ids.body, widget::Canvas::new().color(color::DARK_CHARCOAL)),
                    (
                        ids.footer,
                        widget::Canvas::new()
                            .color(color::DARK_CHARCOAL)
                            .length(100.0),
                    ),
                ])
                .set(ids.master, ui);

            widget::Text::new("Help")
                .color(color::WHITE)
                .font_size(48)
                .top_left_with_margins_on(ids.header, 20.0, 20.0)
                .set(ids.titulo, ui);

            for _click in widget::Button::new()
                .color(color::WHITE)
                .label("Close")
                .label_color(color::BLACK)
                .w_h(60.0, 60.0)
                .bottom_right_with_margins_on(ids.footer, 20.0, 20.0)
                .set(ids.fechar, ui)
            {
                valores.pagina = valores.pagina_anterior;
            }

            widget::Text::new(&ajuda)
                .top_left_with_margins_on(ids.body, 20.0, 20.0)
                .color(color::WHITE)
                .font_size(20)
                .set(ids.texto_ajuda, ui);
        }

        //Página para iniciar o jogo
        fn set_widgets_8(ref mut ui: conrod::UiCell, ids: &mut Ids, valores: &mut Valores) {
            use conrod::{color, widget, Colorable, Labelable, Positionable, Sizeable, Widget};

            widget::Canvas::new()
                .color(conrod::color::DARK_CHARCOAL)
                .set(ids.canvas, ui);

            for _click in widget::Button::new()
                .color(color::WHITE)
                .label("Start Game")
                .label_color(color::BLACK)
                .w_h(
                    ui.w_of(ids.canvas).unwrap() / 5.0,
                    ui.h_of(ids.canvas).unwrap() / 5.0,
                )
                .middle_of(ids.canvas)
                .set(ids.abrir, ui)
            {
                valores.teste.sessao += 1;
                let tipo = match valores.teste.tipo {
                    Tipos::Cenarios(_) => Some(TipoSessao::Cenario(String::new())),
                    Tipos::Leve => Some(TipoSessao::Leve(valores.valores.clone())),
                    Tipos::Vazio => None,
                };
                let mut sessao = Sessao::new(&valores.teste.nome, valores.teste.sessao, tipo);
                sessao.iniciar(&valores.teste, valores.opcao_cenario);
                valores.sessoes.push(sessao);
            }

            for _click in widget::Button::new()
                .bottom_left_with_margins_on(ids.canvas, 10.0, 50.0)
                .color(color::WHITE)
                .label("Return")
                .label_color(color::BLACK)
                .w_h(100.0, 40.0)
                .set(ids.voltar, ui)
            {
                valores.pagina = 5;
            }

            for _click in widget::Button::new()
                .color(color::WHITE)
                .label("?")
                .label_color(color::BLACK)
                .w_h(30.0, 30.0)
                .bottom_left_with_margins_on(ids.canvas, 10.0, 10.0)
                .set(ids.ajuda, ui)
            {
                valores.pagina_anterior = 8;
                valores.pagina = 7;
            }
        }

        pub fn get_data(data: time::Tm) -> String {
            format!(
                "{}/{}/{}-{}:{}:{}",
                data.tm_year + 1900,
                data.tm_mon,
                data.tm_mday,
                data.tm_hour,
                data.tm_min,
                data.tm_sec
            )
        }

        //Sequência de páginas para funções LoL

        // Pagina de criacao de usuário LoL.
        fn set_widgets_9(ref mut ui: conrod::UiCell, ids: &mut Ids, valores: &mut Valores) {
            use conrod::{
                color, widget, Borderable, Colorable, Labelable, Positionable, Sizeable, Widget,
            };

            widget::Canvas::new()
                .flow_down(&[
                    (
                        ids.header,
                        widget::Canvas::new()
                            .length(ui.win_h*2f64/10f64)
                            .color(color::DARK_CHARCOAL)
                            .pad_bottom(20.0)
                            .flow_right(&[
                                (
                                    ids.left_column,
                                    widget::Canvas::new()
                                        .color(color::DARK_CHARCOAL)
                                        .length(ui.win_w/3f64),
                                ),
                                (
                                    ids.right_column,
                                    widget::Canvas::new()
                                        .color(color::DARK_CHARCOAL)
                                        .scroll_kids_vertically()
                                        .length(ui.win_w - ui.win_w/3f64),
                                ),
                            ]),
                    ),
                    (
                        ids.champions,
                        widget::Canvas::new()
                            .color(color::WHITE)
                            .length(ui.win_h*3f64/10f64),
                    ),
                    (
                        ids.entrada,
                        widget::Canvas::new()
                        .color(color::WHITE)
                        .length(ui.win_h*4f64/10f64),
                    ),
                    (
                        ids.footer,
                        widget::Canvas::new().length(ui.win_h/10f64).color(color::BLACK),
                    ),
                ])
                .set(ids.master, ui);

            widget::Text::new("Create your user")
                .top_left_with_margins_on(ids.left_column, 20.0, 20.0)
                .color(color::WHITE)
                .font_size(48)
                .set(ids.titulo, ui);


            for event in widget::TextBox::new(valores.username.as_ref().unwrap())
                .top_left_with_margins_on(ids.left_column, 80.0, 20.0)
                .font_size(20)
                .w_h(320.0, 40.0)
                .border(1.0)
                .border_color(color::BLACK)
                .color(color::WHITE)
                .set(ids.username_edit, ui)
            {
                match event {
                    //widget::text_box::Event::Enter => println!("TextBox: {:?}", valores.nome),
                    //widget::text_box::Event::Update(string) => valores.nome = string.to_string(),
                    widget::text_box::Event::Enter => (),
                    widget::text_box::Event::Update(string) => {
                        valores.username = Some(string.to_string())
                    }
                }
            }

            for _click in widget::Button::new()
                .top_left_with_margins_on(ids.left_column, 80.0, 340.0)
                .color(color::WHITE)
                .label("Check")
                .label_color(color::BLACK)
                .w_h(100.0, 40.0)
                .set(ids.procurar, ui)
            {
                valores.user = LoLU::check_username(valores.username.as_ref().unwrap()).ok().unwrap();
                valores.championMastery = LoLU::get_champions(valores.user.id.as_str()).ok().unwrap();
                ids.champions_images.resize(
                    valores.championMastery.len() as usize,
                    &mut ui.widget_id_generator(),
                );
                valores.lol_user = Some(LoLU::new(valores.user.clone(), Vec::new(), Vec::new(), Settings::new("C:/Riot Games/Riot Client/RiotClientServices.exe")));
            }
            
            // Instantiate the `ListSelect` widget.
            let num_items = valores.championMastery.len();
            let (mut events, scrollbar) = widget::ListSelect::multiple(num_items)
                .flow_right()
                .scrollbar_on_top()
                .w_h(ui.win_w, 128.0)
                //.item_size(148.0)
                .top_left_with_margins_on(ids.champions, 20.0, 20.0)

                .set(ids.champion_list, ui);

            // Handle the `ListSelect`s events.
            let mut i = 0 as usize;
            while let Some(event) = events.next(ui, |i| valores.list_selected.contains(&i)) {
                use widget::list_select::Event;
                match event {
                    // For the `Item` events we instantiate the `List`'s items.
                    Event::Item(item) => {
                        let label_color = match valores.list_selected.contains(&item.i) {
                            true => {
                                conrod::color::RED
                            }
                            false => {
                                conrod::color::WHITE
                            }
                        };
                        let champion_id = valores.championMastery[i].championId as u32;
                        //let (image, label) = valores.champions_image_map.get(champion_id).unwrap();

                        let user_ref = valores.lol_user.as_mut().unwrap();
                        let label = user_ref.settings.get_champion_name(champion_id).unwrap();
                        let image = user_ref.settings.get_champion_image(champion_id, &mut valores.display_data).unwrap();
                        
                        let button = widget::button::Button::image(*image)
                            .border(90.0)
                            .border_color(label_color)
                            .label(&label)
                            .label_color(label_color)
                            .label_font_size(18)
                            .label_y(conrod::position::Relative::Align(conrod::position::Align::Start))
                            //.y_dimension(conrod::position::Dimension::Absolute(128.0))
                            //.x_dimension(conrod::position::Dimension::Absolute(128.0))
                            .w_h(128.0, 128.0);
                        
                        item.set(button, ui);
                        i+= 1;
                    }

                    // The selection has changed.
                    Event::Selection(selection) => {
                        if let conrod::widget::list_select::Selection::Add(list) = selection{
                            for new in list{
                                if valores.list_selected.contains(&new){
                                    valores.list_selected.remove(&new);
                                }else{
                                    valores.list_selected.insert(new);
                                }
                            }
                            
                        }
                        
                    }

                    // The remaining events indicate interactions with the `ListSelect` widget.
                    event => ()//println!("{:?}", &event),
                }
            }

            // Instantiate the scrollbar for the list.
            if let Some(s) = scrollbar {
                s.set(ui);
            }

            for i in 0..5{
                
                let color = match valores.lanes_selected.contains(&i) {
                    true => {
                        conrod::color::RED
                    }
                    false => {
                        conrod::color::BLACK
                    }
                };

                widget::primitive::shape::rectangle::Rectangle::fill_with([128.0, 128.0], color)
                    .color(color)
                    .top_left_with_margins_on(ids.entrada, 20.0, ((i as f64)*148.0) + 20.0)
                    .set(ids.laneBG[i], ui);

                for _click in widget::button::Button::image(valores.lane_sprites[i])
                .border(0.1)
                .border_color(conrod::color::YELLOW)

                .w_h(128.0, 128.0)
                .top_left_with_margins_on(ids.entrada, 20.0, ((i as f64)*148.0) + 20.0)
                .set(ids.lanes[i], ui)
                {
                    
                    if valores.lanes_selected.contains(&i){
                        valores.lanes_selected.remove(&i);
                    }else{
                        valores.lanes_selected.insert(i);
                    }
                }
            }

            
            

            for _click in widget::Button::new()
                .color(color::WHITE)
                .label("?")
                .label_color(color::BLACK)
                .w_h(30.0, 30.0)
                .top_right_with_margins_on(ids.right_column, 10.0, 10.0)
                .set(ids.ajuda, ui)
            {
                valores.pagina_anterior = 8;
                valores.pagina = 7;
            }

            for _click in widget::Button::new()
                .top_left_with_margins_on(ids.footer, 10.0, 50.0)
                .color(conrod::color::WHITE)
                .label("Return")
                .label_color(conrod::color::BLACK)
                .w_h(100.0, 40.0)
                .set(ids.voltar, ui)
            {
                valores.pagina = 0;
            }

            for _click in widget::Button::new()
                .top_right_with_margins_on(ids.footer, 10.0, 10.0)
                .color(conrod::color::WHITE)
                .label("Confirm")
                .label_color(conrod::color::BLACK)
                .w_h(100.0, 40.0)
                .set(ids.pronto, ui)
                {
                    let mut champ_ids: Vec<isize> = Vec::new();
                    for champ_index in &valores.list_selected{
                        champ_ids.push(valores.championMastery[*champ_index].championId);
                    }

                    champ_ids.sort_by(|a, b| a.partial_cmp(b).unwrap());

                    let mut lanes: Vec<usize> = valores.lanes_selected.drain().collect();
                    lanes.sort();

                    //valores.lol_user = Some(LoLU::new(valores.user.clone(), lanes, champ_ids, Settings::new("C:/Riot Games/Riot Client/RiotClientServices.exe")));
                    valores.lol_user.as_mut().unwrap().update_data(lanes, champ_ids);
                    valores.lol_user.as_ref().unwrap().save();

                    load_lol_user(ui, ids, &mut valores.lol_user, valores.username.as_ref().unwrap());

                    valores.pagina = 10;
                };
        }

        // Pagina de usuário aberto.
        fn set_widgets_10(ref mut ui: conrod::UiCell, ids: &mut Ids, valores: &mut Valores) {
            use conrod::{
                color, widget, Borderable, Colorable, Labelable, Positionable, Sizeable, Widget,
            };

            widget::Canvas::new()
                .flow_down(&[
                    (
                        ids.header,
                        widget::Canvas::new()
                            .length(200.0)
                            .color(color::DARK_CHARCOAL)
                            .pad_bottom(20.0),
                    ),
                    (
                        ids.body,
                        widget::Canvas::new().flow_right(&[
                            (
                                ids.left_column,
                                widget::Canvas::new().color(color::DARK_CHARCOAL).scroll_kids(),
                            ),
                            (
                                ids.mid_column,
                                widget::Canvas::new().color(color::DARK_CHARCOAL).scroll_kids(),
                            ),
                            (
                                ids.right_column,
                                widget::Canvas::new().color(color::DARK_CHARCOAL).scroll_kids(),
                            ),
                        ]),
                    ),
                    (
                        ids.footer,
                        widget::Canvas::new().length(60.0).color(color::BLACK),
                    ),
                ])
                .set(ids.master, ui)
            ;

            let mut number = 0;
            for match_data in &valores.lol_user.as_ref().expect("User not loaded").matches{
                let image = valores.lol_user.as_ref().expect("User not loaded").settings.get_champion_image(match_data.champion, &mut valores.display_data).unwrap();
                //let (image, _) = valores.champions_image_map.get(&match_data.champion).unwrap();
                let lane_image = valores.lane_sprites[match_data.role_num()];
    
                match_card(ui, ids, number, image, &lane_image, match_data.match_time, match_data.start_date, match_data.win);
                number += 1;
            }

            let mut number = 0;
            for champion_stats in &valores.lol_user.as_ref().expect("User not loaded").champion_stats{
                //let (image, _) = valores.champions_image_map.get(&champion_stats.champion).unwrap();
                let image = valores.lol_user.as_ref().expect("User not loaded").settings.get_champion_image(champion_stats.champion, &mut valores.display_data).unwrap();
                champion_card(ui, ids, number, image, champion_stats.stats.matches, champion_stats.stats.wins, champion_stats.stats.kda_per_match, champion_stats.stats.kp_per_match*100.0, champion_stats.stats.gold_percentage_per_match*100.0);
                number += 1;
            }

            let mut number = 0;
            for lane_stats in &valores.lol_user.as_ref().expect("User not loaded").lane_stats{
                let lane_image = valores.lane_sprites[lane_stats.role_num()];
                lane_card(ui, ids, number, &lane_image, lane_stats.stats.matches, lane_stats.stats.wins, lane_stats.stats.kda_per_match, lane_stats.stats.kp_per_match*100.0, lane_stats.stats.gold_percentage_per_match*100.0);
                number += 1;
            }
            
            for _click in widget::Button::new()
                .top_right_with_margins_on(ids.footer, 10.0, 10.0)
                .color(color::WHITE)
                .label("Play")
                .label_color(color::BLACK)
                .w_h(100.0, 40.0)
                .set(ids.executar, ui)
            {
                let(s1, r1): (std::sync::mpsc::Sender<Result<(),usize>>, std::sync::mpsc::Receiver<Result<(),usize>>) = mpsc::channel();
                let(s2, r2): (std::sync::mpsc::Sender<LoLSession>, std::sync::mpsc::Receiver<LoLSession>) = mpsc::channel();
                valores.channel_receiver = Some(r1);
                valores.session_receiver = Some(r2);
                valores.lol_user.as_mut().unwrap().start(s1, s2);
                valores.lol_user.as_mut().unwrap().state = LoLState::WAITING;
                valores.pagina = 20;
            };

            for _click in widget::Button::new()
                .top_left_with_margins_on(ids.footer, 10.0, 10.0)
                .color(color::WHITE)
                .label("Edit")
                .label_color(color::BLACK)
                .w_h(100.0, 40.0)
                .set(ids.analise, ui)
            {
                valores.pagina = 14;
            }

            for _click in widget::Button::new()
                .top_left_with_margins_on(ids.footer, 10.0, 120.0)
                .color(color::WHITE)
                .label("Matches")
                .label_color(color::BLACK)
                .w_h(100.0, 40.0)
                .set(ids.processar, ui)
            {
                valores.pagina = 11;
                valores.search_data = SearchData::new();
                valores.found_matches = Vec::new();
                valores.match_selected = None;
                valores.timeline_view = false;
            }

            for _click in widget::Button::new()
                .mid_top_with_margin_on(ids.footer, 10.0)
                .color(color::WHITE)
                .label("Configure")
                .label_color(color::BLACK)
                .w_h(100.0, 40.0)
                .set(ids.exportar, ui)
            {
                valores.pagina = 20;
            }

            for _click in widget::Button::new()
                .top_right_with_margins_on(ids.footer, 10.0, 150.0)
                .color(color::WHITE)
                .label("Baseline")
                .label_color(color::BLACK)
                .w_h(100.0, 40.0)
                .set(ids.baseline, ui)
            {
                valores.pagina = 12;
            }

            for _click in widget::Button::new()
                .top_right_with_margins_on(ids.footer, 10.0, 270.0)
                .color(color::WHITE)
                .label("E4_Data")
                .label_color(color::BLACK)
                .w_h(100.0, 40.0)
                .set(ids.e4_data, ui)
            {
                valores.pagina = 13;
            }

            for _click in widget::Button::new()
                .color(color::WHITE)
                .label("?")
                .label_color(color::BLACK)
                .w_h(30.0, 30.0)
                .top_right_with_margins_on(ids.header, 10.0, 10.0)
                .set(ids.ajuda, ui)
            {
                valores.pagina_anterior = 10;
                valores.pagina = 7;
            }

            widget::Text::new(valores.username.as_ref().unwrap())
                .top_left_with_margins_on(ids.header, 20.0, 20.0)
                .color(color::RED)
                .font_size(48)
                .set(ids.title, ui);

        }

        // Página de visualização de dados
        fn set_widgets_11(ref mut ui: conrod::UiCell, ids: &mut Ids, valores: &mut Valores){
            use conrod::{
                color, widget, Borderable, Colorable, Labelable, Positionable, Sizeable, Widget,
            };

            widget::Canvas::new()
                .flow_down(&[
                    (
                        ids.body,
                        widget::Canvas::new().flow_right(&[
                            (
                                ids.left_column,
                                widget::Canvas::new().length(200.0).flow_down(&[
                                    (
                                        ids.search_bar,
                                        widget::Canvas::new().length(290.0).color(color::DARK_BLUE),
                                    ),
                                    (
                                        ids.all_matches,
                                        widget::Canvas::new().color(color::DARK_CHARCOAL).scroll_kids(),
                                    ),
                                ]),
                            ),
                            (
                                ids.right_column,
                                widget::Canvas::new().color(color::DARK_CHARCOAL),
                            ),
                        ]),
                    ),
                    (
                        ids.footer,
                        widget::Canvas::new().length(60.0).color(color::BLACK),
                    ),
                ])
                .set(ids.master, ui)
            ;

            let champion_search;
            if let Some(name) = &valores.search_data.champion{
                champion_search = name.clone();
            }else{
                champion_search = "Campeões".to_string();
            }

            for event in widget::TextBox::new(&champion_search) //valores.search_options.as_ref().unwrap()
                .mid_top_with_margin_on(ids.search_bar, 0.0)
                .font_size(20)
                .w_h(200.0, 50.0)
                .border(1.0)
                .border_color(color::BLACK)
                .color(color::WHITE)
                .set(ids.search_champion, ui)
            {
                match event {
                    //widget::text_box::Event::Enter => println!("TextBox: {:?}", valores.nome),
                    //widget::text_box::Event::Update(string) => valores.nome = string.to_string(),
                    widget::text_box::Event::Enter => (),
                    widget::text_box::Event::Update(string) => {
                        if string.len() > 0 {
                            valores.search_data.champion = Some(string.to_string());
                        }else{
                            valores.search_data.champion = None;
                        }
                        
                    }
                }
            }

            let team_search;
            if let Some(name) = &valores.search_data.team{
                team_search = name.clone();
            }else{
                team_search = "Team Champions".to_string();
            }

            for event in widget::TextBox::new(&team_search) //valores.search_options.as_ref().unwrap()
                .mid_top_with_margin_on(ids.search_bar, 50.0)
                .font_size(20)
                .w_h(200.0, 50.0)
                .border(1.0)
                .border_color(color::BLACK)
                .color(color::WHITE)
                .set(ids.search_team, ui)
            {
                match event {
                    //widget::text_box::Event::Enter => println!("TextBox: {:?}", valores.nome),
                    //widget::text_box::Event::Update(string) => valores.nome = string.to_string(),
                    widget::text_box::Event::Enter => (),
                    widget::text_box::Event::Update(string) => {
                        valores.search_data.team = Some(string.to_string())
                    }
                }
            }

            let opponents_search;
            if let Some(name) = &valores.search_data.oppontents{
                opponents_search = name.clone();
            }else{
                opponents_search = "Opponents".to_string();
            }

            for event in widget::TextBox::new(&opponents_search) //valores.search_options.as_ref().unwrap()
                .mid_top_with_margin_on(ids.search_bar, 100.0)
                .font_size(20)
                .w_h(200.0, 50.0)
                .border(1.0)
                .border_color(color::BLACK)
                .color(color::WHITE)
                .set(ids.search_opponents, ui)
            {
                match event {
                    //widget::text_box::Event::Enter => println!("TextBox: {:?}", valores.nome),
                    //widget::text_box::Event::Update(string) => valores.nome = string.to_string(),
                    widget::text_box::Event::Enter => (),
                    widget::text_box::Event::Update(string) => {
                        valores.search_data.oppontents = Some(string.to_string())
                    }
                }
            }

            for i in 0..5 as usize{
                
                let color = match valores.search_data.lane == i as isize {
                    true => {
                        conrod::color::RED
                    }
                    false => {
                        conrod::color::BLACK
                    }
                };

                widget::primitive::shape::rectangle::Rectangle::fill_with([30.0, 30.0], color)
                    .color(color)
                    .top_left_with_margins_on(ids.search_bar, 150.0, ((i as f64)*40.0) + 5.0)
                    .set(ids.laneBG[i], ui);

                for _click in widget::button::Button::image(valores.lane_sprites[i])
                .border(0.1)
                .border_color(conrod::color::YELLOW)

                .w_h(30.0, 30.0)
                .top_left_with_margins_on(ids.search_bar, 150.0, ((i as f64)*40.0) + 5.0)
                .set(ids.lanes[i], ui)
                {
                    if valores.search_data.lane != i as isize{
                        valores.search_data.lane = i as isize;
                    }else{
                        valores.search_data.lane = -1;
                    }
                }
            }

            widget::Text::new("Won")
                .top_left_with_margins_on(ids.search_bar, 187.0, 50.0)
                .color(color::WHITE)
                .font_size(18)
                .set(ids.search_won_title, ui);

            let label_win = match valores.search_data.won{
                Some(()) => "X",
                None => "",
            };

            for _click in widget::Button::new()
                .top_left_with_margins_on(ids.search_bar, 187.0, 20.0)
                .color(color::WHITE)
                .label(label_win)
                .label_color(color::BLACK)
                .w_h(30.0, 30.0)
                .set(ids.search_won, ui)
            {
                if valores.search_data.won.is_some(){
                    valores.search_data.won = None;
                }else{
                    valores.search_data.won = Some(());
                }

                if valores.search_data.lost.is_some(){
                    valores.search_data.lost = None;
                }
            }

            widget::Text::new("Lost")
                .top_left_with_margins_on(ids.search_bar, 187.0, 140.0)
                .color(color::WHITE)
                .font_size(18)
                .set(ids.search_lost_title, ui);

            let label_lost = match valores.search_data.lost{
                Some(()) => "X",
                None => "",
            };
            for _click in widget::Button::new()
                .top_left_with_margins_on(ids.search_bar, 185.0, 110.0)
                .color(color::WHITE)
                .label(label_lost)
                .label_color(color::BLACK)
                .w_h(30.0, 30.0)
                .set(ids.search_lost, ui)
            {
                if valores.search_data.lost.is_some(){
                    valores.search_data.lost = None;
                }else{
                    valores.search_data.lost = Some(());
                }

                if valores.search_data.won.is_some(){
                    valores.search_data.won = None;
                }
            }

            widget::Text::new("Has Session")
                .top_left_with_margins_on(ids.search_bar, 227.0, 80.0)
                .color(color::WHITE)
                .font_size(18)
                .set(ids.search_session_title, ui)
            ;

            let label_session = match valores.search_data.session{
                Some(()) => "X",
                None => "",
            };

            for _click in widget::Button::new()
                .top_left_with_margins_on(ids.search_bar, 227.0, 50.0)
                .color(color::WHITE)
                .label(label_session)
                .label_color(color::BLACK)
                .w_h(30.0, 30.0)
                .set(ids.search_session, ui)
            {
                if valores.search_data.session.is_some(){
                    valores.search_data.session = None;
                }else{
                    valores.search_data.session = Some(());
                }
            }

            for _click in widget::Button::new()
                .mid_top_with_margin_on(ids.search_bar, 260.0)
                .color(color::WHITE)
                .label("Go")
                .label_color(color::BLACK)
                .w_h(30.0, 30.0)
                .set(ids.search_button, ui)
            {
                let matches_indexes = valores.lol_user.as_mut().unwrap().find_matches(&valores.search_data).unwrap();
                //let matches_stats_indexes = valores.lol_user.as_mut().unwrap().get_match_stats_db(&matches_indexes).unwrap();
                let len = matches_indexes.len();

                valores.found_matches = matches_indexes;
                
                ids.match_search_result.resize(len, &mut ui.widget_id_generator());
            }

            if valores.found_matches.len() > 0{
                for i in 0..valores.found_matches.len(){
                    //            for each_match in &valores.found_matches{
                    let each_match = &valores.lol_user.as_ref().unwrap().matches[valores.found_matches[i]];
                    let champion = each_match.champion;
                    let role = each_match.role.clone();
                    let start_date = each_match.start_date as i64;
                    let id = each_match._id.clone();

                    let label = format!("{}, {}: \n{}", LoLU::get_champion_from_id(champion, &valores.lol_user.as_ref().unwrap().settings.champion_list).unwrap(), role, LoLU::tm_to_string(LoLU::mili_timestamp_to_tm(start_date)));
                    for _click in widget::Button::new()
                    .mid_top_with_margin_on(ids.all_matches, (i as f64)*50.0)
                    .color(color::WHITE)
                    .center_justify_label()
                    .label(&label)
                    .label_font_size(12)
                    .label_color(color::BLACK)
                    .w_h(200.0, 50.0)
                    .set(ids.match_search_result[i], ui)
                    {
                        valores.match_selected = Some(id.clone());

                        let user = valores.lol_user.as_ref().unwrap();

                        let match_stats = user.matches_stats.iter().find(|each_match| each_match._id == id);
                        let match_timeline = user.timelines.iter().find(|each_timeline| each_timeline._id == id);

                        if match_stats.is_none() || match_timeline.is_none(){
                            let user = valores.lol_user.as_mut().unwrap();
                            user.get_match_stats_db(id.as_str()).unwrap();
                            user.get_match_timeline_db(id.as_str()).unwrap();
                            let user = valores.lol_user.as_ref().unwrap();
                            //let match_found = &user.matches[valores.found_matches[search_index]];
                            valores.found_match = Some(user.matches.iter().find(|each_match| each_match._id == id).unwrap().clone());
                            valores.found_match_stats = Some(user.matches_stats.iter().find(|each_match| each_match._id == id).unwrap().clone());
                            valores.found_match_timeline = Some(user.timelines.iter().find(|each_match| each_match._id == id).unwrap().clone());
                        }else{
                            valores.found_match = Some(user.matches.iter().find(|each_match| each_match._id == id).unwrap().clone());
                            valores.found_match_stats = Some(match_stats.unwrap().clone());
                            valores.found_match_timeline = Some(match_timeline.unwrap().clone());
                        }
                        let user = Some(());
                        valores.plot_view_data = PlotData::new();
                    }
                }
            }

            if let Some(id) = &valores.match_selected{
                let label_view_toggle;
                if valores.timeline_view{
                    label_view_toggle = "To General";
                }else{
                    label_view_toggle = "To Timeline";
                }
                for _click in widget::Button::new()
                    .mid_top_with_margin_on(ids.right_column, 10.0)
                    .color(color::WHITE)
                    .label(label_view_toggle)
                    .label_color(color::BLACK)
                    .w_h(100.0, 30.0)
                    .set(ids.search_toggle_view, ui)
                {
                    valores.timeline_view = !valores.timeline_view;
                }

                let user = valores.lol_user.as_ref().unwrap();
                let user_name = user.user.name.as_str().clone();
                let match_found = valores.found_match.as_ref().unwrap();
                let session = match_found.session;
                let match_stats = valores.found_match_stats.as_ref().unwrap();
                let match_timeline = valores.found_match_timeline.as_ref().unwrap();

                if session > 0{
                    for _click in widget::Button::new()
                        .bottom_right_with_margins_on(ids.footer, 10.0, 50.0)
                        .color(color::WHITE)
                        .label("Export Videos")
                        .label_color(color::BLACK)
                        .w_h(120.0, 40.0)
                        .set(ids.video_export, ui)
                    {
                        LoLU::export_video(user_name, session, match_found.start_date, match_found.match_time, match_found._id.as_str());
                    }
                }

                if valores.timeline_view{
                    match_timeline_canvas(ui, ids, match_found, match_stats, match_timeline, &mut valores.plot_view_data);
                }else{
                    match_info_canvas(ui, ids, match_found, match_stats, &user.settings, &mut valores.display_data);
                }
                
            }
        
            for _click in widget::Button::new()
                .bottom_left_with_margins_on(ids.footer, 10.0, 50.0)
                .color(color::WHITE)
                .label("Return")
                .label_color(color::BLACK)
                .w_h(100.0, 40.0)
                .set(ids.voltar, ui)
            {
                let len = valores.lol_user.as_ref().unwrap().matches.len();
                ids.match_card_outline.resize(len, &mut ui.widget_id_generator());
                ids.match_card_hero.resize(len, &mut ui.widget_id_generator());
                ids.match_card_lane.resize(len, &mut ui.widget_id_generator());
                ids.match_card_more.resize(len, &mut ui.widget_id_generator());
                ids.match_card_start.resize(len, &mut ui.widget_id_generator());
                ids.match_card_start_time.resize(len, &mut ui.widget_id_generator());
                ids.match_card_duration.resize(len, &mut ui.widget_id_generator());
                ids.match_card_season.resize(len, &mut ui.widget_id_generator());
                ids.match_card_result.resize(len, &mut ui.widget_id_generator());

                let len = valores.lol_user.as_ref().unwrap().champion_stats.len();
                ids.champion_card_outline.resize(len, &mut ui.widget_id_generator());
                ids.champion_card_hero.resize(len, &mut ui.widget_id_generator());
                ids.champion_card_more.resize(len, &mut ui.widget_id_generator());
                ids.champion_card_matches.resize(len, &mut ui.widget_id_generator());
                ids.champion_card_wins.resize(len, &mut ui.widget_id_generator());
                ids.champion_card_kpa.resize(len, &mut ui.widget_id_generator());
                ids.champion_card_kp.resize(len, &mut ui.widget_id_generator());
                ids.champion_card_gp.resize(len, &mut ui.widget_id_generator());
                ids.champion_card_matches_text.resize(len, &mut ui.widget_id_generator());
                ids.champion_card_wins_text.resize(len, &mut ui.widget_id_generator());
                ids.champion_card_kpa_text.resize(len, &mut ui.widget_id_generator());
                ids.champion_card_kp_text.resize(len, &mut ui.widget_id_generator());
                ids.champion_card_gp_text.resize(len, &mut ui.widget_id_generator());

                let len = valores.lol_user.as_ref().unwrap().lane_stats.len();
                ids.lane_card_outline.resize(len, &mut ui.widget_id_generator());
                ids.lane_card_lane.resize(len, &mut ui.widget_id_generator());
                ids.lane_card_more.resize(len, &mut ui.widget_id_generator());
                ids.lane_card_matches.resize(len, &mut ui.widget_id_generator());
                ids.lane_card_wins.resize(len, &mut ui.widget_id_generator());
                ids.lane_card_kpa.resize(len, &mut ui.widget_id_generator());
                ids.lane_card_kp.resize(len, &mut ui.widget_id_generator());
                ids.lane_card_gp.resize(len, &mut ui.widget_id_generator());
                ids.lane_card_matches_text.resize(len, &mut ui.widget_id_generator());
                ids.lane_card_wins_text.resize(len, &mut ui.widget_id_generator());
                ids.lane_card_kpa_text.resize(len, &mut ui.widget_id_generator());
                ids.lane_card_kp_text.resize(len, &mut ui.widget_id_generator());
                ids.lane_card_gp_text.resize(len, &mut ui.widget_id_generator());

                valores.pagina = 10;
            }
        
        }

        //Screen to record baseline signals
        fn set_widgets_12(ref mut ui: conrod::UiCell, ids: &mut Ids, valores: &mut Valores){
            use conrod::{color, widget, Colorable, Labelable, Positionable, Sizeable, Widget};

            widget::Canvas::new()
                .color(conrod::color::DARK_CHARCOAL)
                .set(ids.canvas, ui)
            ;

            let mut info_text = "";
            let difference = valores.timer.elapsed().unwrap().as_secs();
            //valores.timer = std::time::SystemTime::now();
            if difference <= 30{
                info_text = "Let's first record your baseline signals. Relax for 5 minutes."
            }else if difference <= 32{
                info_text = "In 5 seconds you will press the button on your wristband."
            }else if difference <= 33{
                info_text = "3";
            }else if difference <= 34{
                info_text = "2";
            }else if difference <= 35{
                info_text = "1";
            }else if difference <= 45{
                info_text = "Press";
            }else if difference <= 90{
                info_text = "Breath";
            }else if difference <= 150{
                info_text = "Relax";
            }else if difference <= 210{
                info_text = "Halfway done";
            }else if difference <= 270{
                info_text = "It's a beautiful day";
            }else if difference <= 330{
                info_text = "One minute left";
            }else if difference <= 390{
                info_text = "Press the button again";
            }else if difference <= 450{
                info_text = "Thank you";
            }else if difference <= 480{
                valores.pagina = 10;
            }

            widget::Text::new(info_text)
                .middle_of(ids.canvas)
                .color(color::RED)
                .font_size(48)
                .set(ids.lolScreen, ui);

            for _click in widget::Button::new()
                .bottom_left_with_margins_on(ids.canvas, 10.0, 50.0)
                .color(color::WHITE)
                .label("Return")
                .label_color(color::BLACK)
                .w_h(100.0, 40.0)
                .set(ids.voltar, ui)
            {
                valores.pagina = 10;
            }
        }
        
        // Selection of E4 files
        fn set_widgets_13(ref mut ui: conrod::UiCell, ids: &mut Ids, valores: &mut Valores) {
            use conrod::{widget, Colorable, Labelable, Positionable, Sizeable, Widget};

            //let ui = &mut ui.set_widgets();
            let directory = find_folder::Search::KidsThenParents(3, 5)
                .for_folder("Empatica Sessions")
                .unwrap();

            widget::Canvas::new()
                .color(conrod::color::DARK_CHARCOAL)
                .set(ids.canvas, ui);

            for event in widget::FileNavigator::all(&directory)// directories(&directory)
                .color(conrod::color::LIGHT_BLUE)
                .font_size(16)
                .wh_of(ids.canvas)
                .middle_of(ids.canvas)
                //.show_hidden_files(true)  // Use this to show hidden files
                .set(ids.file_navigator, ui)
            {
                if let conrod::widget::file_navigator::Event::ChangeSelection(arquivos) = event {
                    if arquivos.len() == 1 {
                        valores.empatica_files_location = Some(format!("{}",arquivos[0].to_str().unwrap()));
                        /*valores.teste = Teste::carregar(
                            arquivos[0].file_stem().unwrap().to_str().unwrap(),
                            arquivos[0].to_str().unwrap(),
                        );*/
                        //valores.nome =format!("{}", arquivos[0].file_stem().unwrap().to_str().unwrap());
                    }
                }
            }

            for _click in widget::Button::new()
                .color(conrod::color::WHITE)
                .label("?")
                .label_color(conrod::color::BLACK)
                .w_h(30.0, 30.0)
                .bottom_left_with_margins_on(ids.canvas, 10.0, 10.0)
                .set(ids.ajuda, ui)
            {
                valores.pagina_anterior = 4;
                valores.pagina = 7;
            }

            for _click in widget::Button::new()
                .bottom_left_with_margins_on(ids.canvas, 10.0, 50.0)
                .color(conrod::color::WHITE)
                .label("Return")
                .label_color(conrod::color::BLACK)
                .w_h(100.0, 40.0)
                .set(ids.voltar, ui)
            {
                valores.pagina = 10;
            }

            for _click in widget::Button::new()
                .bottom_right_with_margins_on(ids.canvas, 50.0, 50.0)
                .color(conrod::color::WHITE)
                .label("Ready")
                .label_color(conrod::color::BLACK)
                .w_h(100.0, 40.0)
                .set(ids.pronto, ui)
            {
                //println!("is some: {}\n has files: {}", valores.empatica_files_location.is_some(), empatica::check_has_files(&valores.empatica_files_location.as_ref().unwrap()));
                if valores.empatica_files_location.is_some() && empatica::check_has_files(&valores.empatica_files_location.as_ref().unwrap()){
                    valores.lol_user.as_mut().unwrap().process_e4_files(&valores.empatica_files_location.as_ref().unwrap()).unwrap();
                    valores.pagina = 10;
                }
            }
        }

        //Settings change
        fn set_widgets_14(ref mut ui: conrod::UiCell, ids: &mut Ids, valores: &mut Valores) {
            use conrod::{widget, Colorable, Labelable, Positionable, Sizeable, Widget};

            widget::Canvas::new()
                .flow_down(&[
                    (
                        ids.header,
                        widget::Canvas::new()
                            .length(200.0)
                            .color(color::DARK_CHARCOAL)
                            .pad_bottom(20.0),
                    ),
                    (
                        ids.body,
                        widget::Canvas::new().flow_right(&[
                            (
                                ids.left_column,
                                widget::Canvas::new().color(color::DARK_CHARCOAL).scroll_kids(),
                            ),
                            (
                                ids.right_column,
                                //widget::Canvas::new().color(color::DARK_CHARCOAL).scroll_kids(),
                                widget::Canvas::new().flow_down(&[
                                    (
                                        ids.exe_location_canvas,
                                        widget::Canvas::new().color(color::DARK_CHARCOAL).length(50.0),
                                    ),
                                    (
                                        ids.exe_search,
                                        widget::Canvas::new().color(color::DARK_CHARCOAL),
                                    ),
                                ]),
                            ),
                        ]),
                    ),
                    (
                        ids.footer,
                        widget::Canvas::new().length(60.0).color(color::BLACK),
                    ),
                ])
                .set(ids.master, ui)
            ;

            //let ui = &mut ui.set_widgets();
            if valores.exe_folder.is_none(){
                valores.exe_folder =  match find_folder::Search::KidsThenParents(5, 7).for_folder("Riot Client"){
                    Ok(directory) => Some(directory),
                    Err(_) => {
                        let mut directory = PathBuf::from(&valores.lol_user.as_ref().unwrap().settings.exe_location);
                        directory.pop();
                        Some(directory)
                    },
                };
            }

            widget::Text::new("Configurações")
                .middle_of(ids.header)
                .color(color::WHITE)
                .font_size(48)
                .set(ids.titulo , ui);

            widget::Text::new("Local do executável:")
                .top_left_with_margins_on(ids.left_column, 50.0, 50.0)
                .color(color::WHITE)
                .font_size(24)
                .set(ids.exe_location_title, ui);

            widget::Text::new(&valores.lol_user.as_ref().unwrap().settings.exe_location)
                .top_left_with_margins_on(ids.left_column, 80.0, 50.0)
                .color(color::RED)
                .font_size(24)
                .set(ids.exe_location, ui);

            for _click in widget::Button::new()
                .top_left_with_margins_on(ids.left_column, 120.0, 50.0)
                .color(conrod::color::WHITE)
                .label("Mudar")
                .label_color(conrod::color::BLACK)
                .w_h(100.0, 40.0)
                .set(ids.change_location, ui)
            {
                valores.changing_location = true;
            }

            if valores.changing_location {
                for event in widget::FileNavigator::all(valores.exe_folder.as_ref().unwrap())
                    .color(conrod::color::LIGHT_BLUE)
                    .font_size(16)
                    .wh_of(ids.exe_search)
                    .middle_of(ids.exe_search)
                    //.show_hidden_files(true)  // Use this to show hidden files
                    .set(ids.navegador_executavel, ui)
                {
                    if let conrod::widget::file_navigator::Event::ChangeSelection(arquivos) = event {
                        
                        if arquivos.len() == 1 {
                            if arquivos[0].is_file() {
                                valores.exe_new_location = Some(arquivos[0].to_str().unwrap().to_string());
                            }
                        }
                    }
                }

                widget::Text::new(valores.exe_folder.as_ref().unwrap().to_str().unwrap())
                    .mid_left_with_margin_on(ids.exe_location_canvas, 10.0)
                    .color(color::RED)
                    .font_size(24)
                    .set(ids.exe_search_title, ui);

                for _click in widget::Button::new()
                    .mid_right_with_margin_on(ids.exe_location_canvas, 10.0)
                    .color(conrod::color::WHITE)
                    .label("Acima")
                    .label_color(conrod::color::BLACK)
                    .w_h(100.0, 40.0)
                    .set(ids.upper_dir, ui)
                    {
                        valores.exe_folder.as_mut().unwrap().pop();
                        //valores.exe_location = Some(valores.exe_location.unwrap().pop());
                    }

                for _click in widget::Button::new()
                    .bottom_right_with_margins_on(ids.exe_search, 10.0, 50.0)
                    .color(conrod::color::WHITE)
                    .label("Confirmar")
                    .label_color(conrod::color::BLACK)
                    .w_h(100.0, 40.0)
                    .set(ids.pronto, ui)
                    {
                        valores.changing_location = false;
                        if let Some(location) = &valores.exe_new_location{
                            valores.lol_user.as_mut().unwrap().settings.exe_location = location.clone();
                        }
                        
                    }
            }

            for _click in widget::Button::new()
                .color(conrod::color::WHITE)
                .label("?")
                .label_color(conrod::color::BLACK)
                .w_h(30.0, 30.0)
                .bottom_left_with_margins_on(ids.footer, 10.0, 10.0)
                .set(ids.ajuda, ui)
            {
                valores.pagina_anterior = 14;
                valores.pagina = 7;
            }

            for _click in widget::Button::new()
                .bottom_left_with_margins_on(ids.footer, 10.0, 50.0)
                .color(conrod::color::WHITE)
                .label("Return")
                .label_color(conrod::color::BLACK)
                .w_h(100.0, 40.0)
                .set(ids.voltar, ui)
            {
                valores.pagina = 10;
            }
        }
        
        fn set_widgets_20(ref mut ui: conrod::UiCell, ids: &mut Ids, valores: &mut Valores){
            use conrod::{color, widget, Colorable, Labelable, Positionable, Sizeable, Widget};

            widget::Canvas::new()
                .color(conrod::color::DARK_CHARCOAL)
                .set(ids.canvas, ui)
            ;

            let mut info_text = "";
            match valores.lol_user.as_ref().unwrap().state{
                LoLState::WAITING => {
                    if let Some(receiver) = &valores.channel_receiver {
                        if let Ok(msg) = receiver.try_recv() {
                            if msg.is_ok(){
                                valores.lol_user.as_mut().unwrap().state = LoLState::COLLECTING;
                                info_text = "Coletando. Aperte CTRL + L e o botão da pulseira para encerrar.";
                                //let(sender1, receiver1): (std::sync::mpsc::Sender<Result<(),usize>>, std::sync::mpsc::Receiver<Result<(),usize>>) = mpsc::channel();
                                //valores.lol_user.as_mut().unwrap().play(sender1);
                                //valores.channel_receiver = Some(receiver1);
                            }   
                        }
                    }
                    info_text = "Esperando o botão na pulseira e o comando CTRL + L e para começarem as filmagens.";
                },
                LoLState::COLLECTING =>{
                    if let Some(receiver) = &valores.channel_receiver {
                        if let Ok(msg) = receiver.try_recv() {
                            if msg.is_ok(){
                                valores.lol_user.as_mut().unwrap().state = LoLState::FINISHED;

                            }   
                        }
                    }
                    info_text = "Coletando";
                },
                LoLState::FINISHED => {
                    info_text = "Finalizando";
                    if let Some(receiver) = &valores.session_receiver {
                        if let Ok(session) = receiver.recv_timeout(Duration::from_secs(1)){
                            valores.lol_user.as_mut().unwrap().sessions.push(session);
                        }
                    }
                    
                    valores.pagina = 10;
                },
                _ => (),
            }

            widget::Text::new(info_text)
                .middle_of(ids.canvas)
                .color(color::RED)
                .font_size(48)
                .set(ids.lolScreen, ui);

            for _click in widget::Button::new()
                .bottom_left_with_margins_on(ids.canvas, 10.0, 50.0)
                .color(color::WHITE)
                .label("Return")
                .label_color(color::BLACK)
                .w_h(100.0, 40.0)
                .set(ids.voltar, ui)
            {
                valores.pagina = 1;
            }

            for _click in widget::Button::new()
                .color(color::WHITE)
                .label("?")
                .label_color(color::BLACK)
                .w_h(30.0, 30.0)
                .bottom_left_with_margins_on(ids.canvas, 10.0, 10.0)
                .set(ids.ajuda, ui)
            {
                valores.pagina_anterior = 9;
                valores.pagina = 7;
            }
        }

        fn match_card(ui: &mut conrod::UiCell, ids: &mut Ids, number: usize, hero_image: &conrod::image::Id, lane_image: &conrod::image::Id, match_duration: i64, match_start: i64, win: bool) {
            use conrod::{
                color, widget, Colorable, Labelable, Positionable, Sizeable, Widget,
            };

            widget::primitive::shape::rectangle::Rectangle::outline([(WIDTH as f64)/3.0, 200.0])
                .color(color::DARK_BLUE)
                .top_left_with_margins_on(ids.left_column, (number as f64*200.0), 0.0)
                .set(ids.match_card_outline[number], ui)
            ;

            widget::Image::new(*hero_image)
                .w_h(160.0, 160.0)
                .mid_left_with_margin_on(ids.match_card_outline[number], 20.0)
                .set(ids.match_card_hero[number], ui)
            ;

            widget::Image::new(*lane_image)
                .w_h(160.0, 160.0)
                .mid_right_with_margin_on(ids.match_card_outline[number], 20.0)
                .set(ids.match_card_lane[number], ui)
            ;


            /*for _click in widget::Button::new()
                .top_right_with_margins_on(ids.match_card_outline[number], 5.0, 5.0)
                .color(color::WHITE)
                .center_justify_label()
                .label("+")
                .label_font_size(24)
                .label_color(color::BLACK)
                .w_h(30.0, 30.0)
                .set(ids.match_card_more[number], ui)
            {
                valores.pagina = 11;
                valores.search_data = SearchData::new();
                valores.found_matches = Vec::new();
                valores.match_selected = Some(id);
                valores.timeline_view = false;
            }*/

            let date = LoLU::tm_to_string(LoLU::mili_timestamp_to_tm(match_start as i64));
            widget::Text::new(date.get(0..10).expect("Error getting date string"))
                .top_left_with_margins_on(ids.match_card_outline[number], 30.0, 200.0)
                .color(color::WHITE)
                .font_size(24)
                .set(ids.match_card_start[number], ui)
            ;

            widget::Text::new(&format!("{}:{}", date.get(11..13).unwrap(), date.get(14..16).unwrap()))
                .top_left_with_margins_on(ids.match_card_outline[number], 60.0, 200.0)
                .color(color::WHITE)
                .font_size(24)
                .set(ids.match_card_start_time[number], ui)
            ;


            widget::Text::new(&LoLU::duration_to_string(match_duration))
                .bottom_left_with_margins_on(ids.match_card_outline[number], 30.0, 200.0)
                .color(color::WHITE)
                .font_size(24)
                .set(ids.match_card_duration[number], ui)
            ;

            let result = match win{
                true => "Won",
                false => "Lost",
            };
            widget::Text::new(result)
                .mid_right_with_margin_on(ids.match_card_outline[number], 180.0)
                .color(color::WHITE)
                .font_size(24)
                .set(ids.match_card_result[number], ui)
            ;
        }

        fn champion_card(ui: &mut conrod::UiCell, ids: &mut Ids, number: usize, hero_image: &conrod::image::Id, matches: i64, wins: i64, kda: f64, kp: f64, gp: f64){
            use conrod::{
                color, widget, Colorable, Labelable, Positionable, Sizeable, Widget,
            };

            widget::primitive::shape::rectangle::Rectangle::outline([(WIDTH as f64)/3.0, 200.0])
                .color(color::DARK_BLUE)
                .top_left_with_margins_on(ids.mid_column, (number as f64)*200.0, 0.0)
                .set(ids.champion_card_outline[number], ui)
            ;

            widget::Image::new(*hero_image)
                .w_h(160.0, 160.0)
                .mid_left_with_margin_on(ids.champion_card_outline[number], 20.0)
                .set(ids.champion_card_hero[number], ui)
            ;


            /*for _click in widget::Button::new()
                .top_right_with_margins_on(ids.champion_card_outline[number], 5.0, 5.0)
                .color(color::WHITE)
                .center_justify_label()
                .label("+")
                .label_font_size(24)
                .label_color(color::BLACK)
                .w_h(30.0, 30.0)
                .set(ids.champion_card_more[number], ui)
            {
                
            }*/

            widget::Text::new("Matches:")
                .top_left_with_margins_on(ids.champion_card_outline[number], 20.0, 200.0)
                .color(color::WHITE)
                .font_size(24)
                .set(ids.champion_card_matches_text[number], ui)
            ;

            widget::Text::new(&matches.to_string())
                .top_left_with_margins_on(ids.champion_card_outline[number], 20.0, 500.0)
                .color(color::WHITE)
                .font_size(24)
                .set(ids.champion_card_matches[number], ui)
            ;

            widget::Text::new("Wins:")
                .top_left_with_margins_on(ids.champion_card_outline[number], 52.0, 200.0)
                .color(color::WHITE)
                .font_size(24)
                .set(ids.champion_card_wins_text[number], ui)
            ;

            widget::Text::new(&wins.to_string())
                .top_left_with_margins_on(ids.champion_card_outline[number], 52.0, 500.0)
                .color(color::WHITE)
                .font_size(24)
                .set(ids.champion_card_wins[number], ui)
            ;

            widget::Text::new("KDA/Match:")
                .top_left_with_margins_on(ids.champion_card_outline[number], 84.0, 200.0)
                .color(color::WHITE)
                .font_size(24)
                .set(ids.champion_card_kpa_text[number], ui)
            ;

            widget::Text::new(&format!("{:.2}", kda))
                .top_left_with_margins_on(ids.champion_card_outline[number], 84.0, 500.0)
                .color(color::WHITE)
                .font_size(24)
                .set(ids.champion_card_kpa[number], ui)
            ;

            widget::Text::new("KP/Match:")
                .top_left_with_margins_on(ids.champion_card_outline[number], 116.0, 200.0)
                .color(color::WHITE)
                .font_size(24)
                .set(ids.champion_card_kp_text[number], ui)
            ;

            widget::Text::new(&format!("{:.2}%", kp))
                .top_left_with_margins_on(ids.champion_card_outline[number], 116.0, 500.0)
                .color(color::WHITE)
                .font_size(24)
                .set(ids.champion_card_kp[number], ui)
            ;

            widget::Text::new("Gold Participation/Match:")
                .top_left_with_margins_on(ids.champion_card_outline[number], 148.0, 200.0)
                .color(color::WHITE)
                .font_size(24)
                .set(ids.champion_card_gp_text[number], ui)
            ;

            widget::Text::new(&format!("{:.2}%", gp))
                .top_left_with_margins_on(ids.champion_card_outline[number], 148.0, 500.0)
                .color(color::WHITE)
                .font_size(24)
                .set(ids.champion_card_gp[number], ui)
            ;

            
        }

        fn lane_card(ui: &mut conrod::UiCell, ids: &mut Ids, number: usize, lane_image: &conrod::image::Id, matches: i64, wins: i64, kda: f64, kp: f64, gp: f64){
            use conrod::{
                color, widget, Colorable, Labelable, Positionable, Sizeable, Widget,
            };

            widget::primitive::shape::rectangle::Rectangle::outline([(WIDTH as f64)/3.0, 200.0])
                .color(color::DARK_BLUE)
                .top_left_with_margins_on(ids.right_column, (number as f64)*200.0, 0.0)
                .set(ids.lane_card_outline[number], ui)
            ;

            widget::Image::new(*lane_image)
                .w_h(160.0, 160.0)
                .mid_left_with_margin_on(ids.lane_card_outline[number], 20.0)
                .set(ids.lane_card_lane[number], ui)
            ;


            /*for _click in widget::Button::new()
                .top_right_with_margins_on(ids.lane_card_outline[number], 5.0, 5.0)
                .color(color::WHITE)
                .center_justify_label()
                .label("+")
                .label_font_size(24)
                .label_color(color::BLACK)
                .w_h(30.0, 30.0)
                .set(ids.lane_card_more[number], ui)
            {
                
            }*/

            widget::Text::new("Matches:")
                .top_left_with_margins_on(ids.lane_card_outline[number], 20.0, 200.0)
                .color(color::WHITE)
                .font_size(24)
                .set(ids.lane_card_matches_text[number], ui)
            ;

            widget::Text::new(&matches.to_string())
                .top_left_with_margins_on(ids.lane_card_outline[number], 20.0, 500.0)
                .color(color::WHITE)
                .font_size(24)
                .set(ids.lane_card_matches[number], ui)
            ;

            widget::Text::new("Wins:")
                .top_left_with_margins_on(ids.lane_card_outline[number], 52.0, 200.0)
                .color(color::WHITE)
                .font_size(24)
                .set(ids.lane_card_wins_text[number], ui)
            ;

            widget::Text::new(&wins.to_string())
                .top_left_with_margins_on(ids.lane_card_outline[number], 52.0, 500.0)
                .color(color::WHITE)
                .font_size(24)
                .set(ids.lane_card_wins[number], ui)
            ;

            widget::Text::new("KDA/Match:")
                .top_left_with_margins_on(ids.lane_card_outline[number], 84.0, 200.0)
                .color(color::WHITE)
                .font_size(24)
                .set(ids.lane_card_kpa_text[number], ui)
            ;

            widget::Text::new(&format!("{:.2}", kda))
                .top_left_with_margins_on(ids.lane_card_outline[number], 84.0, 500.0)
                .color(color::WHITE)
                .font_size(24)
                .set(ids.lane_card_kpa[number], ui)
            ;

            widget::Text::new("KP/Match:")
                .top_left_with_margins_on(ids.lane_card_outline[number], 116.0, 200.0)
                .color(color::WHITE)
                .font_size(24)
                .set(ids.lane_card_kp_text[number], ui)
            ;

            widget::Text::new(&format!("{:.2}%", kp))
                .top_left_with_margins_on(ids.lane_card_outline[number], 116.0, 500.0)
                .color(color::WHITE)
                .font_size(24)
                .set(ids.lane_card_kp[number], ui)
            ;

            widget::Text::new("Gold Participation/Match:")
                .top_left_with_margins_on(ids.lane_card_outline[number], 148.0, 200.0)
                .color(color::WHITE)
                .font_size(24)
                .set(ids.lane_card_gp_text[number], ui)
            ;

            widget::Text::new(&format!("{:.2}%", gp))
                .top_left_with_margins_on(ids.lane_card_outline[number], 148.0, 500.0)
                .color(color::WHITE)
                .font_size(24)
                .set(ids.lane_card_gp[number], ui)
            ;

            
        }

        fn match_info_canvas(ui: &mut conrod::UiCell, ids: &mut Ids, match_data: &MatchData, match_stats: &MatchStats, settings: &Settings, display_data: &mut DisplayData){
            use conrod::{
                color, widget, Colorable, Labelable, Positionable, Sizeable, Widget,
            };

            let player_team_stats;
            let opposing_team_stats;

            if match_data.team_id == match_stats.stats[0].teamId as i64{
                player_team_stats = &match_stats.stats[0];
                opposing_team_stats = &match_stats.stats[1];
            }else{
                player_team_stats = &match_stats.stats[1];
                opposing_team_stats = &match_stats.stats[0];
            }
            let mut player_team_kills = 0;
            let mut opposing_team_kills = 0;
            let mut player_team_first_kill = false;
            for player in &match_stats.participant_stats{
                if player.teamId == match_data.team_id as i64{
                    player_team_kills += player.kills;
                    if player.firstBloodKill{
                        player_team_first_kill = true;
                    }
                }else{
                    opposing_team_kills += player.kills;
                }
            }

            draw_team_stats(ui, ids, &player_team_stats.objectives, player_team_kills, 0.0, match_data.win, player_team_stats.teamId, player_team_first_kill);
            draw_team_stats(ui, ids, &opposing_team_stats.objectives, opposing_team_kills, 1000.0, !match_data.win, opposing_team_stats.teamId, !player_team_first_kill);
            let mut player_team_number = 0;
            let mut opposing_team_number = 0;
            
            for player in &match_stats.participant_stats{
                let player_name = player.summonerName.as_str();

                if player.teamId == match_data.team_id as i64{
                    draw_player_stats(ui, ids, player, player_team_number, player_name, 0.0, settings, display_data);
                    player_team_number += 1;
                }else{
                    
                    draw_player_stats(ui, ids, player, opposing_team_number, player_name, 1000.0, settings, display_data);
                    opposing_team_number += 1;
                }
            }
        }

        fn match_timeline_canvas(ui: &mut conrod::UiCell, ids: &mut Ids, match_data: &MatchData, match_stats: &MatchStats, timeline: &MatchTimeline, plot_data: &mut PlotData){
            use conrod::{
                color, widget, Colorable, Labelable, Positionable, Sizeable, Widget
            };
            use std::iter::once;
            
            let min_x = 0.0;
            let max_x = 10.0;
            let min_y = 0.0;
            let max_y = plot_data.max_value;

            let grid_w = 1200.0;
            let grid_h = 800.0;

            let quarter_lines = widget::grid::Lines::step(5.0_f64).thickness(2.0);
            let sixteenth_lines = widget::grid::Lines::step(1.0_f64).thickness(1.0);
            let lines = &[
                quarter_lines.x(),
                quarter_lines.y(),
                sixteenth_lines.x(),
                sixteenth_lines.y(),
            ];
            
            widget::Grid::new(min_x, max_x, min_y, max_y, lines.iter().cloned())
                .color(color::rgb(0.1, 0.12, 0.15))
                .w_h(grid_w, grid_h)
                .mid_top_with_margin_on(ids.right_column, 50.0)
                .set(ids.grid, ui)
            ;

            let label_texts_len = (plot_data.max_value/5.0).ceil() as usize + 1;
            let label_step_size = grid_h/(label_texts_len - 1) as f64;
            ids.grid_label.resize(label_texts_len, &mut ui.widget_id_generator());
            for text_i in 0..label_texts_len{
                widget::Text::new((text_i*5).to_string().as_str())
                    .bottom_left_with_margins_on(ids.grid, (text_i as f64)*label_step_size - 10.0, -40.0)
                    .color(color::WHITE)
                    .font_size(24)
                    .set(ids.grid_label[text_i], ui)
            ;
            }
            
            if match_data.session != 0{
                if timeline.hr.len() > 0{
                    widget::Text::new("Heart Rate")
                        .top_left_with_margins_on(ids.right_column, 170.0, 40.0)
                        .color(color::WHITE)
                        .font_size(24)
                        .set(ids.hr_button_label, ui)
                    ;
                    let hr_label;
                    let hr_color;
                    match plot_data.hr{
                        Some(color) => {
                            hr_label = "x";
                            hr_color = PlotData::get_color(color);
                            if plot_data.hr_values.len() <= 0 {
                                plot_data.hr_values = MatchTimeline::get_empatica_plot_points(timeline.hr.iter().collect(), match_data.start_date, match_data.match_time, grid_w, grid_h, true).unwrap();
                            }
                            
                            widget::PointPath::centred(plot_data.hr_values.clone().into_iter())
                                .bottom_left_of(ids.grid)
                                .color(hr_color)
                                .w_h(grid_w, grid_h)
                                .set(ids.hr_plot, ui)
                            ;
                        },
                        None => {
                            hr_label = "";
                            hr_color = color::BLACK;
                        },
                    };
                    for _click in widget::Button::new()
                        .top_left_with_margins_on(ids.right_column, 175.0, 10.0)
                        .color(color::WHITE)
                        .label(hr_label)
                        .label_color(hr_color)
                        .w_h(20.0, 20.0)
                        .set(ids.hr_button, ui)
                    {
                        plot_data.hr = match plot_data.hr{
                            Some(color) => {
                                if color > 17{
                                    None
                                }else{
                                    Some(color + 1)
                                }
                            },
                            None => {
                                Some(0)
                            },
                        };
                    }
                }

                if timeline.eda.len() > 0 {
                    widget::Text::new("GSR")
                        .top_left_with_margins_on(ids.right_column, 200.0, 40.0)
                        .color(color::WHITE)
                        .font_size(24)
                        .set(ids.gsr_button_label, ui)
                    ;
                    let gsr_label;
                    let gsr_color;
                    match plot_data.gsr{
                        Some(color) => {
                            gsr_label = "x";
                            gsr_color = PlotData::get_color(color);
                            if plot_data.gsr_values.len() <= 0 {
                                plot_data.gsr_values = MatchTimeline::get_empatica_plot_points(timeline.eda.iter().collect(), match_data.start_date, match_data.match_time, grid_w, grid_h, true).unwrap();
                            }
                            
                            widget::PointPath::centred(plot_data.gsr_values.clone().into_iter())
                                .bottom_left_of(ids.grid)
                                .color(gsr_color)
                                .w_h(grid_w, grid_h)
                                .set(ids.gsr_plot, ui)
                            ;
                        },
                        None => {
                            gsr_label = "";
                            gsr_color = color::BLACK;
                        },
                    };
                    for _click in widget::Button::new()
                        .top_left_with_margins_on(ids.right_column, 205.0, 10.0)
                        .color(color::WHITE)
                        .label(gsr_label)
                        .label_color(gsr_color)
                        .w_h(20.0, 20.0)
                        .set(ids.gsr_button, ui)
                    {
                        plot_data.gsr = match plot_data.gsr{
                            Some(color) => {
                                if color > 17{
                                    None
                                }else{
                                    Some(color + 1)
                                }
                            },
                            None => {
                                Some(0)
                            },
                        };
                    }
                }

                if timeline.bvp.len() > 0 {
                    widget::Text::new("BVP")
                        .top_left_with_margins_on(ids.right_column, 230.0, 40.0)
                        .color(color::WHITE)
                        .font_size(24)
                        .set(ids.bvp_button_label, ui)
                    ;
                    let bvp_label;
                    let bvp_color;
                    match plot_data.bvp{
                        Some(color) => {
                            bvp_label = "x";
                            bvp_color = PlotData::get_color(color);
                            if plot_data.bvp_values.len() <= 0 {
                                plot_data.bvp_values = MatchTimeline::get_empatica_plot_points(timeline.bvp.iter().collect(), match_data.start_date, match_data.match_time, grid_w, grid_h, true).unwrap();
                            }
                            
                            widget::PointPath::centred(plot_data.bvp_values.clone().into_iter())
                                .bottom_left_of(ids.grid)
                                .color(bvp_color)
                                .w_h(grid_w, grid_h)
                                .set(ids.bvp_plot, ui)
                            ;
                        },
                        None => {
                            bvp_label = "";
                            bvp_color = color::BLACK;
                        },
                    };
                    for _click in widget::Button::new()
                        .top_left_with_margins_on(ids.right_column, 235.0, 10.0)
                        .color(color::WHITE)
                        .label(bvp_label)
                        .label_color(bvp_color)
                        .w_h(20.0, 20.0)
                        .set(ids.bvp_button, ui)
                    {
                        plot_data.bvp = match plot_data.bvp{
                            Some(color) => {
                                if color > 17{
                                    None
                                }else{
                                    Some(color + 1)
                                }
                            },
                            None => {
                                Some(0)
                            },
                        };
                    }
                }

                if timeline.facial_inferings.len() > 0 {
                    widget::Text::new("Happiness")
                        .top_left_with_margins_on(ids.right_column, 260.0, 40.0)
                        .color(color::WHITE)
                        .font_size(24)
                        .set(ids.happiness_button_label, ui)
                    ;
                    let happiness_label;
                    let happiness_color;
                    match plot_data.happiness{
                        Some(color) => {
                            happiness_label = "x";
                            happiness_color = PlotData::get_color(color);
                            if plot_data.happiness_values.len() <= 0 {
                                //let filter: Vec<LoLData> = timeline.facial_inferings.iter().filter(|x| &x.name == "Felicidade").cloned().collect();
                                let filter: Vec<&LoLData> = timeline.facial_inferings.iter().filter(|x| &x.name == "Felicidade").collect();
                                //plot_data.happiness_values = MatchTimeline::get_empatica_plot_points(&timeline.facial_inferings, match_data.start_date, match_data.match_time, grid_, grid_).unwrap();
                                plot_data.happiness_values = MatchTimeline::get_empatica_plot_points(filter, match_data.start_date, match_data.match_time, grid_w, grid_h, false).unwrap();
                            }
                            
                            widget::PointPath::centred(plot_data.happiness_values.clone().into_iter())
                                .bottom_left_of(ids.grid)
                                .color(happiness_color)
                                .w_h(grid_w, grid_h)
                                .set(ids.happiness_plot, ui)
                            ;
                        },
                        None => {
                            happiness_label = "";
                            happiness_color = color::BLACK;
                        },
                    };
                    for _click in widget::Button::new()
                        .top_left_with_margins_on(ids.right_column, 265.0, 10.0)
                        .color(color::WHITE)
                        .label(happiness_label)
                        .label_color(happiness_color)
                        .w_h(20.0, 20.0)
                        .set(ids.happiness_button, ui)
                    {
                        plot_data.happiness = match plot_data.happiness{
                            Some(color) => {
                                if color > 17{
                                    None
                                }else{
                                    Some(color + 1)
                                }
                            },
                            None => {
                                Some(0)
                            },
                        };
                    }

                    widget::Text::new("Sadness")
                        .top_left_with_margins_on(ids.right_column, 290.0, 40.0)
                        .color(color::WHITE)
                        .font_size(24)
                        .set(ids.sadness_button_label, ui)
                    ;
                    let sadness_label;
                    let sadness_color;
                    match plot_data.sadness{
                        Some(color) => {
                            sadness_label = "x";
                            sadness_color = PlotData::get_color(color);
                            if plot_data.sadness_values.len() <= 0 {
                                let filter: Vec<&LoLData> = timeline.facial_inferings.iter().filter(|x| &x.name == "Tristeza").collect();
                                plot_data.sadness_values = MatchTimeline::get_empatica_plot_points(filter, match_data.start_date, match_data.match_time, grid_w, grid_h, false).unwrap();
                            }
                            
                            widget::PointPath::centred(plot_data.sadness_values.clone().into_iter())
                                .bottom_left_of(ids.grid)
                                .color(sadness_color)
                                .w_h(grid_w, grid_h)
                                .set(ids.sadness_plot, ui)
                            ;
                        },
                        None => {
                            sadness_label = "";
                            sadness_color = color::BLACK;
                        },
                    };
                    for _click in widget::Button::new()
                        .top_left_with_margins_on(ids.right_column, 295.0, 10.0)
                        .color(color::WHITE)
                        .label(sadness_label)
                        .label_color(sadness_color)
                        .w_h(20.0, 20.0)
                        .set(ids.sadness_button, ui)
                    {
                        plot_data.sadness = match plot_data.sadness{
                            Some(color) => {
                                if color > 17{
                                    None
                                }else{
                                    Some(color + 1)
                                }
                            },
                            None => {
                                Some(0)
                            },
                        };
                    }

                    widget::Text::new("Anger")
                        .top_left_with_margins_on(ids.right_column, 320.0, 40.0)
                        .color(color::WHITE)
                        .font_size(24)
                        .set(ids.anger_button_label, ui)
                    ;
                    let anger_label;
                    let anger_color;
                    match plot_data.anger{
                        Some(color) => {
                            anger_label = "x";
                            anger_color = PlotData::get_color(color);
                            if plot_data.anger_values.len() <= 0 {
                                let filter: Vec<&LoLData> = timeline.facial_inferings.iter().filter(|x| &x.name == "Raiva").collect();
                                plot_data.anger_values = MatchTimeline::get_empatica_plot_points(filter, match_data.start_date, match_data.match_time, grid_w, grid_h, false).unwrap();
                            }
                            
                            widget::PointPath::centred(plot_data.anger_values.clone().into_iter())
                                .bottom_left_of(ids.grid)
                                .color(anger_color)
                                .w_h(grid_w, grid_h)
                                .set(ids.anger_plot, ui)
                            ;
                        },
                        None => {
                            anger_label = "";
                            anger_color = color::BLACK;
                        },
                    };
                    for _click in widget::Button::new()
                        .top_left_with_margins_on(ids.right_column, 325.0, 10.0)
                        .color(color::WHITE)
                        .label(anger_label)
                        .label_color(anger_color)
                        .w_h(20.0, 20.0)
                        .set(ids.anger_button, ui)
                    {
                        plot_data.anger = match plot_data.anger{
                            Some(color) => {
                                if color > 17{
                                    None
                                }else{
                                    Some(color + 1)
                                }
                            },
                            None => {
                                Some(0)
                            },
                        };
                    }

                    widget::Text::new("Fear")
                        .top_left_with_margins_on(ids.right_column, 350.0, 40.0)
                        .color(color::WHITE)
                        .font_size(24)
                        .set(ids.fear_button_label, ui)
                    ;
                    let fear_label;
                    let fear_color;
                    match plot_data.fear{
                        Some(color) => {
                            fear_label = "x";
                            fear_color = PlotData::get_color(color);
                            if plot_data.fear_values.len() <= 0 {
                                let filter: Vec<&LoLData> = timeline.facial_inferings.iter().filter(|x| &x.name == "medo").collect();
                                plot_data.fear_values = MatchTimeline::get_empatica_plot_points(filter, match_data.start_date, match_data.match_time, grid_w, grid_h, false).unwrap();
                            }
                            
                            widget::PointPath::centred(plot_data.fear_values.clone().into_iter())
                                .bottom_left_of(ids.grid)
                                .color(fear_color)
                                .w_h(grid_w, grid_h)
                                .set(ids.fear_plot, ui)
                            ;
                        },
                        None => {
                            fear_label = "";
                            fear_color = color::BLACK;
                        },
                    };
                    for _click in widget::Button::new()
                        .top_left_with_margins_on(ids.right_column, 355.0, 10.0)
                        .color(color::WHITE)
                        .label(fear_label)
                        .label_color(fear_color)
                        .w_h(20.0, 20.0)
                        .set(ids.fear_button, ui)
                    {
                        plot_data.fear = match plot_data.fear{
                            Some(color) => {
                                if color > 17{
                                    None
                                }else{
                                    Some(color + 1)
                                }
                            },
                            None => {
                                Some(0)
                            },
                        };
                    }

                    widget::Text::new("Surprise")
                        .top_left_with_margins_on(ids.right_column, 380.0, 40.0)
                        .color(color::WHITE)
                        .font_size(24)
                        .set(ids.surprise_button_label, ui)
                    ;
                    let surprise_label;
                    let surprise_color;
                    match plot_data.surprise{
                        Some(color) => {
                            surprise_label = "x";
                            surprise_color = PlotData::get_color(color);
                            if plot_data.surprise_values.len() <= 0 {
                                let filter: Vec<&LoLData> = timeline.facial_inferings.iter().filter(|x| &x.name == "Surpresa").collect();
                                plot_data.surprise_values = MatchTimeline::get_empatica_plot_points(filter, match_data.start_date, match_data.match_time, grid_w, grid_h, false).unwrap();
                            }
                            
                            widget::PointPath::centred(plot_data.surprise_values.clone().into_iter())
                                .bottom_left_of(ids.grid)
                                .color(surprise_color)
                                .w_h(grid_w, grid_h)
                                .set(ids.surprise_plot, ui)
                            ;
                        },
                        None => {
                            surprise_label = "";
                            surprise_color = color::BLACK;
                        },
                    };
                    for _click in widget::Button::new()
                        .top_left_with_margins_on(ids.right_column, 385.0, 10.0)
                        .color(color::WHITE)
                        .label(surprise_label)
                        .label_color(surprise_color)
                        .w_h(20.0, 20.0)
                        .set(ids.surprise_button, ui)
                    {
                        plot_data.surprise = match plot_data.surprise{
                            Some(color) => {
                                if color > 17{
                                    None
                                }else{
                                    Some(color + 1)
                                }
                            },
                            None => {
                                Some(0)
                            },
                        };
                    }

                    widget::Text::new("Disgust")
                        .top_left_with_margins_on(ids.right_column, 410.0, 40.0)
                        .color(color::WHITE)
                        .font_size(24)
                        .set(ids.disgust_button_label, ui)
                    ;
                    let disgust_label;
                    let disgust_color;
                    match plot_data.disgust{
                        Some(color) => {
                            disgust_label = "x";
                            disgust_color = PlotData::get_color(color);
                            if plot_data.disgust_values.len() <= 0 {
                                let filter: Vec<&LoLData> = timeline.facial_inferings.iter().filter(|x| &x.name == "Desgosto").collect();
                                plot_data.disgust_values = MatchTimeline::get_empatica_plot_points(filter, match_data.start_date, match_data.match_time, grid_w, grid_h, false).unwrap();
                            }
                            
                            widget::PointPath::centred(plot_data.disgust_values.clone().into_iter())
                                .bottom_left_of(ids.grid)
                                .color(disgust_color)
                                .w_h(grid_w, grid_h)
                                .set(ids.disgust_plot, ui)
                            ;
                        },
                        None => {
                            disgust_label = "";
                            disgust_color = color::BLACK;
                        },
                    };
                    for _click in widget::Button::new()
                        .top_left_with_margins_on(ids.right_column, 415.0, 10.0)
                        .color(color::WHITE)
                        .label(disgust_label)
                        .label_color(disgust_color)
                        .w_h(20.0, 20.0)
                        .set(ids.disgust_button, ui)
                    {
                        plot_data.disgust = match plot_data.disgust{
                            Some(color) => {
                                if color > 17{
                                    None
                                }else{
                                    Some(color + 1)
                                }
                            },
                            None => {
                                Some(0)
                            },
                        };
                    }
                }

                /*
                pub hapinness: Option<i64>,
                pub hapinness_values: Vec<[f64;2]>,
                pub sadness: Option<i64>,
                pub sadness_values: Vec<[f64;2]>,
                pub anger: Option<i64>,
                pub anger_values: Vec<[f64;2]>,
                pub fear: Option<i64>,
                pub fear_values: Vec<[f64;2]>,
                pub surprise: Option<i64>,
                pub surprise_values: Vec<[f64;2]>,
                pub disgust: Option<i64>,
                pub disgust_values: Vec<[f64;2]>,
                */

            }
            

            widget::Text::new("Kills")
                .top_left_with_margins_on(ids.right_column, 20.0, 40.0)
                .color(color::WHITE)
                .font_size(24)
                .set(ids.kills_button_label, ui)
            ;
            let kills_label;
            let kills_color;
            match plot_data.kills{
                Some(color) => {
                    kills_label = "x";
                    kills_color = PlotData::get_color(color);
                    if plot_data.kills_values.len() <= 0 {
                        plot_data.kills_values = timeline.get_match_plot_points(PlotableValues::Kills, match_data.start_date, match_data.match_time, grid_w, grid_h, &mut plot_data.max_value, match_data.team_id).unwrap();
                    }
                    
                    widget::PointPath::centred(plot_data.kills_values.clone().into_iter())
                        .bottom_left_of(ids.grid)
                        .color(kills_color)
                        .w_h(grid_w, grid_h)
                        .set(ids.kills_plot, ui)
                    ;
                },
                None => {
                    kills_label = "";
                    kills_color = color::BLACK;
                },
            };
            for _click in widget::Button::new()
                .top_left_with_margins_on(ids.right_column, 25.0, 10.0)
                .color(color::WHITE)
                .label(kills_label)
                .label_color(kills_color)
                .w_h(20.0, 20.0)
                .set(ids.kills_button, ui)
            {
                plot_data.kills = match plot_data.kills{
                    Some(color) => {
                        if color > 17{
                            None
                        }else{
                            Some(color + 1)
                        }
                    },
                    None => {
                        Some(0)
                    },
                };
            }

            widget::Text::new("Deaths")
                .top_left_with_margins_on(ids.right_column, 50.0, 40.0)
                .color(color::WHITE)
                .font_size(24)
                .set(ids.deaths_button_label, ui)
            ;
            let deaths_label;
            let deaths_color;
            match plot_data.deaths{
                Some(color) => {
                    deaths_label = "x";
                    deaths_color = PlotData::get_color(color);
                    if plot_data.deaths_values.len() <= 0 {
                        plot_data.deaths_values = timeline.get_match_plot_points(PlotableValues::Deaths, match_data.start_date, match_data.match_time, grid_w, grid_h, &mut plot_data.max_value, match_data.team_id).unwrap();
                    }
                    
                    widget::PointPath::centred(plot_data.deaths_values.clone().into_iter())
                        .bottom_left_of(ids.grid)
                        .color(deaths_color)
                        .w_h(grid_w, grid_h)
                        .set(ids.deaths_plot, ui)
                    ;
                },
                None => {
                    deaths_label = "";
                    deaths_color = color::BLACK;
                },
            };
            for _click in widget::Button::new()
                .top_left_with_margins_on(ids.right_column, 55.0, 10.0)
                .color(color::WHITE)
                .label(deaths_label)
                .label_color(deaths_color)
                .w_h(20.0, 20.0)
                .set(ids.deaths_button, ui)
            {
                plot_data.deaths = match plot_data.deaths{
                    Some(color) => {
                        if color > 17{
                            None
                        }else{
                            Some(color + 1)
                        }
                    },
                    None => {
                        Some(0)
                    },
                };
            }

            widget::Text::new("Assists")
                .top_left_with_margins_on(ids.right_column, 80.0, 40.0)
                .color(color::WHITE)
                .font_size(24)
                .set(ids.assists_button_label, ui)
            ;
            let assists_label;
            let assists_color;
            match plot_data.assists{
                Some(color) => {
                    assists_label = "x";
                    assists_color = PlotData::get_color(color);
                    if plot_data.assists_values.len() <= 0 {
                        plot_data.assists_values = timeline.get_match_plot_points(PlotableValues::Assists, match_data.start_date, match_data.match_time, grid_w, grid_h, &mut plot_data.max_value, match_data.team_id).unwrap();
                    }
                    
                    widget::PointPath::centred(plot_data.assists_values.clone().into_iter())
                        .bottom_left_of(ids.grid)
                        .color(assists_color)
                        .w_h(grid_w, grid_h)
                        .set(ids.assists_plot, ui)
                    ;
                },
                None => {
                    assists_label = "";
                    assists_color = color::BLACK;
                },
            };
            for _click in widget::Button::new()
                .top_left_with_margins_on(ids.right_column, 85.0, 10.0)
                .color(color::WHITE)
                .label(assists_label)
                .label_color(assists_color)
                .w_h(20.0, 20.0)
                .set(ids.assists_button, ui)
            {
                plot_data.assists = match plot_data.assists{
                    Some(color) => {
                        if color > 17{
                            None
                        }else{
                            Some(color + 1)
                        }
                    },
                    None => {
                        Some(0)
                    },
                };
            }

            widget::Text::new("Barons")
                .top_left_with_margins_on(ids.right_column, 110.0, 40.0)
                .color(color::WHITE)
                .font_size(24)
                .set(ids.barons_button_label, ui)
            ;
            let barons_label;
            let barons_color;
            match plot_data.barons{
                Some(color) => {
                    barons_label = "x";
                    barons_color = PlotData::get_color(color);
                    if plot_data.barons_values.len() <= 0 {
                        plot_data.barons_values = timeline.get_match_plot_points(PlotableValues::Barons, match_data.start_date, match_data.match_time, grid_w, grid_h, &mut plot_data.max_value, match_data.team_id).unwrap();
                    }
                    
                    widget::PointPath::centred(plot_data.barons_values.clone().into_iter())
                        .bottom_left_of(ids.grid)
                        .color(barons_color)
                        .w_h(grid_w, grid_h)
                        .set(ids.barons_plot, ui)
                    ;
                },
                None => {
                    barons_label = "";
                    barons_color = color::BLACK;
                },
            };
            for _click in widget::Button::new()
                .top_left_with_margins_on(ids.right_column, 115.0, 10.0)
                .color(color::WHITE)
                .label(barons_label)
                .label_color(barons_color)
                .w_h(20.0, 20.0)
                .set(ids.barons_button, ui)
            {
                plot_data.barons = match plot_data.barons{
                    Some(color) => {
                        if color > 17{
                            None
                        }else{
                            Some(color + 1)
                        }
                    },
                    None => {
                        Some(0)
                    },
                };
            }

            widget::Text::new("Dragons")
                .top_left_with_margins_on(ids.right_column, 140.0, 40.0)
                .color(color::WHITE)
                .font_size(24)
                .set(ids.dragons_button_label, ui)
            ;
            let dragons_label;
            let dragons_color;
            match plot_data.dragons{
                Some(color) => {
                    dragons_label = "x";
                    dragons_color = PlotData::get_color(color);
                    if plot_data.dragons_values.len() <= 0 {
                        plot_data.dragons_values = timeline.get_match_plot_points(PlotableValues::Dragons, match_data.start_date, match_data.match_time, grid_w, grid_h, &mut plot_data.max_value, match_data.team_id).unwrap();
                    }
                    
                    widget::PointPath::centred(plot_data.dragons_values.clone().into_iter())
                        .bottom_left_of(ids.grid)
                        .color(dragons_color)
                        .w_h(grid_w, grid_h)
                        .set(ids.dragons_plot, ui)
                    ;
                },
                None => {
                    dragons_label = "";
                    dragons_color = color::BLACK;
                },
            };
            for _click in widget::Button::new()
                .top_left_with_margins_on(ids.right_column, 145.0, 10.0)
                .color(color::WHITE)
                .label(dragons_label)
                .label_color(dragons_color)
                .w_h(20.0, 20.0)
                .set(ids.dragons_button, ui)
            {
                plot_data.dragons = match plot_data.dragons{
                    Some(color) => {
                        if color > 17{
                            None
                        }else{
                            Some(color + 1)
                        }
                    },
                    None => {
                        Some(0)
                    },
                };
            }
            
            /*let left = [0.0, 0.0];
            let top = [125.0, 500.0];
            let right = [550.0, 250.0];
            
            let points = once(left).chain(once(top)).chain(once(right));

            widget::PointPath::centred(points)
                .bottom_left_of(ids.grid)
                .w_h(500.0, 500.0)
                .set(ids.point_path, ui);*/
        }

        fn draw_team_stats(ui: &mut conrod::UiCell, ids: &mut Ids, team_stats: &ObjectivesDto, kills: i64, offset: f64, win: bool, team_id: i64, first_kill: bool){
            use conrod::{
                color, widget, Colorable, Labelable, Positionable, Sizeable, Widget,
            };

            let result;
            if win{
                result = "Won";
            }else{
                result = "Lost";
            }

            let index;
            if offset < 1.0 {
                index = 0;
            }else{
                index = 1;
            }

            widget::Text::new(&format!("Result: {}", result))
                .top_left_with_margins_on(ids.right_column, 20.0, 20.0 + offset)
                .color(color::WHITE)
                .font_size(24)
                .set(ids.match_result[index], ui)
            ;

            let color = match team_id{
                100 => "Blue",
                _ => "Red",
            };
            widget::Text::new(&format!("Team Color: {}", color))
                .top_left_with_margins_on(ids.right_column, 20.0, 300.0 + offset)
                .color(color::WHITE)
                .font_size(24)
                .set(ids.match_team_color[index], ui)
            ;
            widget::Text::new(&format!("First Blood: {}", first_kill))
                .top_left_with_margins_on(ids.right_column, 40.0, 20.0 + offset)
                .color(color::WHITE)
                .font_size(24)
                .set(ids.first_blood[index], ui)
            ;
            widget::Text::new(&format!("Team Kills: {}", kills))
                .top_left_with_margins_on(ids.right_column, 40.0, 300.0 + offset)
                .color(color::WHITE)
                .font_size(24)
                .set(ids.team_kills[index], ui)
            ;
            widget::Text::new(&format!("First Tower: {}", team_stats.tower.first))
                .top_left_with_margins_on(ids.right_column, 60.0, 20.0 + offset)
                .color(color::WHITE)
                .font_size(24)
                .set(ids.first_tower[index], ui)
            ;
            widget::Text::new(&format!("Tower Kills: {}", team_stats.tower.kills))
                .top_left_with_margins_on(ids.right_column, 60.0, 300.0 + offset)
                .color(color::WHITE)
                .font_size(24)
                .set(ids.tower_kills[index], ui)
            ;
            widget::Text::new(&format!("First Inhibitor: {}", team_stats.inhibitor.first))
                .top_left_with_margins_on(ids.right_column, 80.0, 20.0 + offset)
                .color(color::WHITE)
                .font_size(24)
                .set(ids.first_inhibitor[index], ui)
            ;
            widget::Text::new(&format!("Inhibitor Kills: {}", team_stats.inhibitor.kills))
                .top_left_with_margins_on(ids.right_column, 80.0, 300.0 + offset)
                .color(color::WHITE)
                .font_size(24)
                .set(ids.inhibitor_kills[index], ui)
            ;
            widget::Text::new(&format!("First Dragon: {}", team_stats.dragon.first))
                .top_left_with_margins_on(ids.right_column, 100.0, 20.0 + offset)
                .color(color::WHITE)
                .font_size(24)
                .set(ids.first_dragon[index], ui)
            ;
            widget::Text::new(&format!("Dragon Kills: {}", team_stats.dragon.kills))
                .top_left_with_margins_on(ids.right_column, 100.0, 300.0 + offset)
                .color(color::WHITE)
                .font_size(24)
                .set(ids.dragon_kills[index], ui)
            ;
            widget::Text::new(&format!("First Baron: {}", team_stats.baron.first))
                .top_left_with_margins_on(ids.right_column, 120.0, 20.0 + offset)
                .color(color::WHITE)
                .font_size(24)
                .set(ids.first_baron[index], ui)
            ;
            widget::Text::new(&format!("Baron Kills: {}", team_stats.baron.kills))
                .top_left_with_margins_on(ids.right_column, 120.0, 300.0 + offset)
                .color(color::WHITE)
                .font_size(24)
                .set(ids.baron_kills[index], ui)
            ;
            widget::Text::new(&format!("First Rift Herald: {}", team_stats.riftHerald.first))
                .top_left_with_margins_on(ids.right_column, 140.0, 20.0 + offset)
                .color(color::WHITE)
                .font_size(24)
                .set(ids.first_herald[index], ui)
            ;
            widget::Text::new(&format!("Rift Herald Kills: {}", team_stats.riftHerald.kills))
                .top_left_with_margins_on(ids.right_column, 140.0, 300.0 + offset)
                .color(color::WHITE)
                .font_size(24)
                .set(ids.herald_kills[index], ui)
            ;
        }

        fn draw_player_stats(ui: &mut conrod::UiCell, ids: &mut Ids, player_stats: &ParticipantDto, player_number: i64, player_name: &str, offset: f64, settings: &Settings, display_data: &mut DisplayData){
            use conrod::{
                color, widget, Colorable, Labelable, Positionable, Sizeable, Widget,
            };
            let team = (offset/1000.0) as i64 ;
            //Imagem do campeão
            let image = settings.get_champion_image(player_stats.championId as u32, display_data).unwrap();
            widget::Image::new(*image)
                .w_h(100.0, 100.0)
                .top_left_with_margins_on(ids.right_column,  (player_number as f64 * 120.0) + 200.0, 10.0 + offset)
                .set(ids.player_image[(player_number + 5*team) as usize], ui)
            ;
            //Level final
            widget::Text::new(&format!("Lv. {}", player_stats.champLevel))
                .top_left_with_margins_on(ids.right_column, (player_number as f64 * 120.0) + 270.0, 120.0 + offset)
                .color(color::WHITE)
                .font_size(24)
                .set(ids.player_level[(player_number + 5*team) as usize], ui)
            ;
            //Player name
            widget::Text::new(&format!("{}", player_name))
                .top_left_with_margins_on(ids.right_column, (player_number as f64 * 120.0) + 200.0, 120.0 + offset)
                .color(color::WHITE)
                .font_size(24)
                .set(ids.player_name[(player_number + 5*team) as usize], ui)
            ;
            //Kills
            widget::Text::new(&format!("{}", player_stats.kills))
                .top_left_with_margins_on(ids.right_column, (player_number as f64 * 120.0) + 200.0, 400.0 + offset)
                .color(color::WHITE)
                .font_size(24)
                .set(ids.player_kills[(player_number + 5*team) as usize], ui)
            ;
            //Deaths
            widget::Text::new(&format!("{}", player_stats.deaths))
                .top_left_with_margins_on(ids.right_column, (player_number as f64 * 120.0) + 200.0, 450.0 + offset)
                .color(color::WHITE)
                .font_size(24)
                .set(ids.player_deaths[(player_number + 5*team) as usize], ui)
            ;
            //Assists
            widget::Text::new(&format!("{}", player_stats.assists))
                .top_left_with_margins_on(ids.right_column, (player_number as f64 * 120.0) + 200.0, 500.0 + offset)
                .color(color::WHITE)
                .font_size(24)
                .set(ids.player_assists[(player_number + 5*team) as usize], ui)
            ;
            //Ouro ganho
            widget::Text::new(&format!("{:2},{:0<3}", player_stats.goldEarned/1000, player_stats.goldEarned%1000))
                .top_left_with_margins_on(ids.right_column, (player_number as f64 * 120.0) + 200.0, 550.0 + offset)
                .color(color::WHITE)
                .font_size(24)
                .set(ids.player_gold[(player_number + 5*team) as usize], ui)
            ;
            //Itens comprados
            if player_stats.item0 != 0{
                let image = settings.get_item_image(player_stats.item0 as u32, display_data);
                widget::Image::new(*image)
                    .w_h(20.0, 20.0)
                    .top_left_with_margins_on(ids.right_column,  (player_number as f64 * 120.0) + 270.0, 400.0 + offset)
                    .set(ids.player_itens[0 + 7*(player_number + 5*team) as usize], ui)
                ;
            }
            if player_stats.item1 != 0{
                let image = settings.get_item_image(player_stats.item1 as u32, display_data);
                widget::Image::new(*image)
                    .w_h(20.0, 20.0)
                    .top_left_with_margins_on(ids.right_column,  (player_number as f64 * 120.0) + 270.0, 425.0 + offset)
                    .set(ids.player_itens[1 + 7*(player_number + 5*team) as usize], ui)
                ;
            }
            if player_stats.item2 != 0{
                let image = settings.get_item_image(player_stats.item2 as u32, display_data);
                widget::Image::new(*image)
                    .w_h(20.0, 20.0)
                    .top_left_with_margins_on(ids.right_column,  (player_number as f64 * 120.0) + 270.0, 450.0 + offset)
                    .set(ids.player_itens[2 + 7*(player_number + 5*team) as usize], ui)
                ;
            }
            if player_stats.item3 != 0{
                let image = settings.get_item_image(player_stats.item3 as u32, display_data);
                widget::Image::new(*image)
                    .w_h(20.0, 20.0)
                    .top_left_with_margins_on(ids.right_column,  (player_number as f64 * 120.0) + 270.0, 475.0 + offset)
                    .set(ids.player_itens[3 + 7*(player_number + 5*team) as usize], ui)
                ;
            }
            if player_stats.item4 != 0{
                let image = settings.get_item_image(player_stats.item4 as u32, display_data);
                widget::Image::new(*image)
                    .w_h(20.0, 20.0)
                    .top_left_with_margins_on(ids.right_column,  (player_number as f64 * 120.0) + 270.0, 500.0 + offset)
                    .set(ids.player_itens[4 + 7*(player_number + 5*team) as usize], ui)
                ;
            }
            if player_stats.item5 != 0{
                let image = settings.get_item_image(player_stats.item5 as u32, display_data);
                widget::Image::new(*image)
                    .w_h(20.0, 20.0)
                    .top_left_with_margins_on(ids.right_column,  (player_number as f64 * 120.0) + 270.0, 525.0 + offset)
                    .set(ids.player_itens[5 + 7*(player_number + 5*team) as usize], ui)
                ;
            }
            if player_stats.item6 != 0{
                let image = settings.get_item_image(player_stats.item6 as u32, display_data);
                widget::Image::new(*image)
                    .w_h(20.0, 20.0)
                    .top_left_with_margins_on(ids.right_column,  (player_number as f64 * 120.0) + 270.0, 550.0 + offset)
                    .set(ids.player_itens[6 + 7*(player_number + 5*team) as usize], ui)
                ;
            }
        }

        fn load_lol_user(ui: &mut conrod::UiCell, ids: &mut Ids, lol_user: &mut Option<LoLU>, username: &str){
            *lol_user = Some(LoLU::load(username));
            //LoLU::get_matches_id(&valores.lol_user.as_ref().unwrap().user.accountId);

            let len = lol_user.as_ref().unwrap().matches.len();
            ids.match_card_outline.resize(len, &mut ui.widget_id_generator());
            ids.match_card_hero.resize(len, &mut ui.widget_id_generator());
            ids.match_card_lane.resize(len, &mut ui.widget_id_generator());
            ids.match_card_more.resize(len, &mut ui.widget_id_generator());
            ids.match_card_start.resize(len, &mut ui.widget_id_generator());
            ids.match_card_start_time.resize(len, &mut ui.widget_id_generator());
            ids.match_card_duration.resize(len, &mut ui.widget_id_generator());
            ids.match_card_season.resize(len, &mut ui.widget_id_generator());
            ids.match_card_result.resize(len, &mut ui.widget_id_generator());

            let len = lol_user.as_ref().unwrap().champion_stats.len();
            ids.champion_card_outline.resize(len, &mut ui.widget_id_generator());
            ids.champion_card_hero.resize(len, &mut ui.widget_id_generator());
            ids.champion_card_more.resize(len, &mut ui.widget_id_generator());
            ids.champion_card_matches.resize(len, &mut ui.widget_id_generator());
            ids.champion_card_wins.resize(len, &mut ui.widget_id_generator());
            ids.champion_card_kpa.resize(len, &mut ui.widget_id_generator());
            ids.champion_card_kp.resize(len, &mut ui.widget_id_generator());
            ids.champion_card_gp.resize(len, &mut ui.widget_id_generator());
            ids.champion_card_matches_text.resize(len, &mut ui.widget_id_generator());
            ids.champion_card_wins_text.resize(len, &mut ui.widget_id_generator());
            ids.champion_card_kpa_text.resize(len, &mut ui.widget_id_generator());
            ids.champion_card_kp_text.resize(len, &mut ui.widget_id_generator());
            ids.champion_card_gp_text.resize(len, &mut ui.widget_id_generator());

            let len = lol_user.as_ref().unwrap().lane_stats.len();
            ids.lane_card_outline.resize(len, &mut ui.widget_id_generator());
            ids.lane_card_lane.resize(len, &mut ui.widget_id_generator());
            ids.lane_card_more.resize(len, &mut ui.widget_id_generator());
            ids.lane_card_matches.resize(len, &mut ui.widget_id_generator());
            ids.lane_card_wins.resize(len, &mut ui.widget_id_generator());
            ids.lane_card_kpa.resize(len, &mut ui.widget_id_generator());
            ids.lane_card_kp.resize(len, &mut ui.widget_id_generator());
            ids.lane_card_gp.resize(len, &mut ui.widget_id_generator());
            ids.lane_card_matches_text.resize(len, &mut ui.widget_id_generator());
            ids.lane_card_wins_text.resize(len, &mut ui.widget_id_generator());
            ids.lane_card_kpa_text.resize(len, &mut ui.widget_id_generator());
            ids.lane_card_kp_text.resize(len, &mut ui.widget_id_generator());
            ids.lane_card_gp_text.resize(len, &mut ui.widget_id_generator());
        }


        // Generate a unique `WidgetId` for each widget.
        widget_ids! {
            struct Ids {
                canvas,
                master,
                header,
                footer,
                body,
                left_column,
                mid_column,
                right_column,
                title,

                voltar,
                abrir,
                criar,
                navegador_executavel,
                pronto,
                procurar,
                file_navigator,
                titulo,
                nome_edit,
                executavel_edit,
                tipos,
                num_vals,
                num_cens,
                vars[],
                vars_nome[],
                cens[],
                cens_nome[],
                vals[],
                vals_nome[],
                ok,
                entrada,
                executar,
                exportar,
                processar,
                editar,
                executavel,
                tipo,
                sessoes[],
                xis,
                data,
                ypslom,
                rectangle[],
                selecionar[],
                selecionar_saidas[],
                rectangle_saida[],
                tipo_aux,
                title_aux,
                executavel_aux,
                texto_cenario,
                texto_variaveis,
                aleatorio,
                readme,
                fechar,
                ajuda,
                texto_ajuda,
                numero,
                analise,

                load,
                cancel,
                lol_beta,
                lol,
                username_edit,
                champions,
                champions_images[],
                champion_list,
                champion_list_item,
                lanes[],
                laneBG[],
                lolScreen,

                change_location,
                exe_location_title,
                exe_location,
                upper_dir,
                exe_search,
                exe_search_title,
                exe_location_canvas,

                match_card_outline[],
                match_card_hero[],
                match_card_lane[],
                match_card_more[],
                match_card_start[],
                match_card_start_time[],
                match_card_duration[],
                match_card_season[],
                match_card_result[],

                champion_card_outline[],
                champion_card_hero[],
                champion_card_more[],
                champion_card_matches_text[],
                champion_card_wins_text[],
                champion_card_kpa_text[],
                champion_card_kp_text[],
                champion_card_gp_text[],
                champion_card_matches[],
                champion_card_wins[],
                champion_card_kpa[],
                champion_card_kp[],
                champion_card_gp[],

                lane_card_outline[],
                lane_card_lane[],
                lane_card_more[],
                lane_card_matches_text[],
                lane_card_wins_text[],
                lane_card_kpa_text[],
                lane_card_kp_text[],
                lane_card_gp_text[],
                lane_card_matches[],
                lane_card_wins[],
                lane_card_kpa[],
                lane_card_kp[],
                lane_card_gp[],

                all_matches,
                search_bar,
                search_input,
                search_button,
                search_champion,
                search_team,
                search_opponents,
                search_lane,
                search_won,
                search_won_title,
                search_lost,
                search_lost_title,
                search_session,
                search_session_title,
                match_search_result[],

                video_export,

                grid_label[],
                search_toggle_view,
                match_result[],
                match_team_color[],
                first_blood[],
                team_kills[],
                first_tower[],
                tower_kills[],
                first_inhibitor[],
                inhibitor_kills[],
                first_dragon[],
                dragon_kills[],
                first_baron[],
                baron_kills[],
                first_herald[],
                herald_kills[],

                player_image[],
                player_level[],
                player_name[],
                player_kills[],
                player_deaths[],
                player_assists[],
                player_gold[],
                player_itens[],

                grid,
                point_path,
                kills_plot,
                kills_button,
                kills_button_label,
                deaths_plot,
                deaths_button,
                deaths_button_label,
                assists_plot,
                assists_button,
                assists_button_label,
                barons_plot,
                barons_button,
                barons_button_label,
                dragons_plot,
                dragons_button,
                dragons_button_label,
                hr_plot,
                hr_button,
                hr_button_label,
                gsr_plot,
                gsr_button,
                gsr_button_label,
                bvp_plot,
                bvp_button,
                bvp_button_label,
                happiness_plot,
                happiness_button,
                happiness_button_label,
                sadness_plot,
                sadness_button,
                sadness_button_label,
                anger_plot,
                anger_button,
                anger_button_label,
                fear_plot,
                fear_button,
                fear_button_label,
                surprise_plot,
                surprise_button,
                surprise_button_label,
                disgust_plot,
                disgust_button,
                disgust_button_label,

                baseline,
                e4_data,
            }
        }
    }
}

impl Valores {
    pub fn fill(&mut self) {
        match self.teste.tipo {
            Tipos::Vazio => (),
            Tipos::Leve => {
                self.variaveis = self.teste.variaveis.clone();
            }
            Tipos::Cenarios(ref cenarios) => {
                self.variaveis = self.teste.variaveis.clone();
                self.cenarios.clear();
                self.valores.clear();
                for cenario in cenarios {
                    self.cenarios.push(cenario.nome.clone());
                    for valor in cenario.variaveis.iter() {
                        self.valores.push(valor.as_string());
                    }
                }
            }
        }
    }

    pub fn zerar(&mut self) {
        self.teste = Teste {
            nome: "Name".to_string(),
            executavel: "Executable".to_string(),
            variaveis: Vec::new(),
            tipo: Tipos::Vazio,
            sessao: 0,
        };
        self.sessoes = Vec::new();
        self.sessoes_novas = Vec::new();
        self.opcao = None;
        self.opcao_export = None;
        self.opcao_saidas = vec![false, false, false, false, false, false, false];
        self.opcao_cenario = None;
        self.valores = Vec::new();
        self.variaveis = Vec::new();
        self.num_vals = "0".to_string();
        self.num_cens = "0".to_string();
        self.cenarios = Vec::new();
        //executavel: "Executavel".to_string(),
        self.executavel_caminho = None;
        self.pagina = 1;
        self.selecionado = 9999;
        self.ajuda = 0;
        self.pagina_anterior = 0;
    }
}

pub struct SearchData{
    pub champion: Option<String>,
    pub team: Option<String>,
    pub lane: isize,
    pub oppontents: Option<String>,
    pub won: Option<()>,
    pub lost: Option<()>,
    pub session: Option<()>,
}

impl SearchData{
    pub fn new() -> SearchData{
        SearchData{
            champion: None,
            team: None,
            lane: -1,
            oppontents: None,
            won: None,
            lost: None,
            session: None,
        }
    }
}

pub struct DisplayData{
    pub champions_image_map: HashMap<u32, conrod::image::Id>,
    pub itens_image_map: HashMap<u32, conrod::image::Id>,
    pub display: conrod::glium::Display,
    pub image_map: conrod::image::Map<conrod::glium::Texture2d>,
}

pub struct PlotData{
    pub times: Option<(i64, i64)>,
    pub max_value: f64,

    pub kills: Option<i64>,
    pub kills_values: Vec<[f64;2]>,
    pub deaths: Option<i64>,
    pub deaths_values: Vec<[f64;2]>,
    pub assists: Option<i64>,
    pub assists_values: Vec<[f64;2]>,
    pub barons: Option<i64>,
    pub barons_values: Vec<[f64;2]>,
    pub dragons: Option<i64>,
    pub dragons_values: Vec<[f64;2]>,
    pub hr: Option<i64>,
    pub hr_values: Vec<[f64;2]>,
    pub gsr: Option<i64>,
    pub gsr_values: Vec<[f64;2]>,
    pub bvp: Option<i64>,
    pub bvp_values: Vec<[f64;2]>,
    pub happiness: Option<i64>,
    pub happiness_values: Vec<[f64;2]>,
    pub sadness: Option<i64>,
    pub sadness_values: Vec<[f64;2]>,
    pub anger: Option<i64>,
    pub anger_values: Vec<[f64;2]>,
    pub fear: Option<i64>,
    pub fear_values: Vec<[f64;2]>,
    pub surprise: Option<i64>,
    pub surprise_values: Vec<[f64;2]>,
    pub disgust: Option<i64>,
    pub disgust_values: Vec<[f64;2]>,
}
impl PlotData{
    pub fn new() -> PlotData{
        PlotData{
            times: None,
            max_value: 10.0,

            kills: None,
            kills_values: Vec::new(),
            deaths: None,
            deaths_values: Vec::new(),
            assists: None,
            assists_values: Vec::new(),
            barons: None,
            barons_values: Vec::new(),
            dragons: None,
            dragons_values: Vec::new(),
            hr: None,
            hr_values: Vec::new(),
            gsr: None,
            gsr_values: Vec::new(),
            bvp: None,
            bvp_values: Vec::new(),
            happiness: None,
            happiness_values: Vec::new(),
            sadness: None,
            sadness_values: Vec::new(),
            anger: None,
            anger_values: Vec::new(),
            fear: None,
            fear_values: Vec::new(),
            surprise: None,
            surprise_values: Vec::new(),
            disgust: None,
            disgust_values: Vec::new(),
        }
    }

    pub fn get_color(color_num:i64)->color::Color{
        match color_num{
            0=> color::BLUE,
            1=> color::GREEN,
            2=> color::RED,
            3=> color::YELLOW,
            4=> color::ORANGE,
            5=> color::PURPLE,
            6=> color::DARK_BLUE,
            7=> color::DARK_GREEN,
            8=> color::DARK_RED,
            9=> color::DARK_YELLOW,
            10=> color::DARK_ORANGE,
            11=> color::DARK_PURPLE,
            12=> color::LIGHT_BLUE,
            13=> color::LIGHT_GREEN,
            14=> color::LIGHT_RED,
            15=> color::DARK_YELLOW,
            16=> color::DARK_ORANGE,
            17=> color::DARK_PURPLE,
            _=> color::WHITE,
        }
    }
}