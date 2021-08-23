//Comando para executar o exemplo "cargo run --release --features "winit glium" --example file_navigator"

extern crate time;

use crate::cenario::Cenario;
use crate::sessao::{Sessao, TipoSessao};
use crate::support;
use crate::teste::{Teste, Tipos};
use crate::lol::{LoLU, User, LoLState, Settings, LoLSession};
use crate::lol_structs::ChampionMasteryDTO;

use conrod;
use conrod::backend::glium::glium;
use conrod::backend::glium::glium::Surface;
use conrod::backend::winit;
use find_folder;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;
use std::path::PathBuf;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::hash_map::RandomState;
use device_query::{DeviceQuery, DeviceState, Keycode};
use std::time::{Duration, Instant};
use std::sync::mpsc;

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
    }
}
/*
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
    pagina: u8,
    selecionado: u32,
    ajuda: u8,
    pagina_anterior: u8,

    load: bool,
    lol_user: Option<LoLU>,
    username: Option<String>,
    user: User,
    championMastery: Vec<ChampionMasteryDTO>,
    images_loaded: bool,
    champions_image_map: HashMap<String, (conrod::image::Id, String)>,
    list_selected: HashSet<usize, RandomState>,
    lane_sprites:  Vec<conrod::image::Id>,
    lanes_selected: HashSet<usize, RandomState>,
    channel_receiver: Option<std::sync::mpsc::Receiver<Result<(),usize>>>,
    session_receiver: Option<std::sync::mpsc::Receiver<LoLSession>>,
}
*/
pub struct GuiLoL {
    pagina_anterior: u8,

    load: bool,
    lol_user: Option<LoLU>,
    username: Option<String>,
    user: User,
    championMastery: Vec<ChampionMasteryDTO>,
    images_loaded: bool,
    champions_image_map: HashMap<String, (conrod::image::Id, String)>,
    list_selected: HashSet<usize, RandomState>,
    lane_sprites:  Vec<conrod::image::Id>,
    lanes_selected: HashSet<usize, RandomState>,
    channel_receiver: Option<std::sync::mpsc::Receiver<Result<(),usize>>>,
    session_receiver: Option<std::sync::mpsc::Receiver<LoLSession>>,
}

impl GuiLoL {
    pub fn new() -> GuiLoL {
        GuiLoL {
            pagina_anterior: 0,

            load: false,
            lol_user: None,
            username: Some("Intsuyou".to_string()),
            user: User::new(),
            championMastery: Vec::new(),
            images_loaded: false,
            champions_image_map: HashMap::new(),
            list_selected: HashSet::new(),
            lane_sprites: Vec::new(),
            lanes_selected: HashSet::new(),
            channel_receiver: None,
            session_receiver: None,

        }
    }

    // Generate a unique `WidgetId` for each widget.

    pub fn run(&mut self) {
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

        let mut current_page = 0;
        // Poll events from the window.
        let mut event_loop = support::EventLoop::new();
        'main: loop {
            // Handle all events.
            for event in event_loop.next(&mut events_loop) {
                // Use the `winit` backend feature to convert the winit event to a conrod one.
                if let Some(event) = winit::convert_event(event.clone(), &display) {
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
            if current_page == 0{
                current_page = self.set_widgets_0(ui.set_widgets(), ids, &mut image_map, &display)
            }

            // Render the `Ui` and then display it on the screen.
            if let Some(primitives) = ui.draw_if_changed() {
                renderer.fill(&display, primitives, &image_map);
                let mut target = display.draw();
                target.clear_color(0.0, 0.0, 0.0, 1.0);
                renderer.draw(&display, &mut target, &image_map).unwrap();
                target.finish().unwrap();
            }
        }
    }

    // Página para entrar na versão para jogos gerais ou LoL.
    fn set_widgets_0(&mut self, ref mut ui: conrod::UiCell, ids: &mut Ids, image_map: &mut conrod::image::Map<conrod::glium::Texture2d>, display: &glium::Display) -> usize {
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
            //return 1;
            return 0;
        }

        for _click in widget::Button::new()
            .color(color::WHITE)
            .label("LoL")
            .label_color(color::BLACK)
            .w_h(100.0, 60.0)
            .middle_of(ids.right_column)
            .set(ids.criar, ui)
        {
            if self.load && self.username.is_some() {
                self.lol_user = Some(LoLU::load(self.username.as_ref().unwrap()));
                //LoLU::get_matches_id(&self.lol_user.as_ref().unwrap().user.accountId);
                return 10;
                
            } else{
                self.username = Some(format!("Intsuyou"));
                if !self.images_loaded{
                    self.champions_image_map = LoLU::get_champions_images(image_map, display);
                    self.images_loaded = true;
                }
                return 9;
            }
        }

        if !self.load{
            for _click in widget::Button::new()
                .color(color::WHITE)
                .label("Load?")
                .label_color(color::BLACK)
                .w_h(100.0, 60.0)
                .mid_bottom_with_margin_on(ids.right_column, 50.0)
                .set(ids.load, ui)
            {
                self.load = true;
            }
        }
        

        if self.load {
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
                        self.username = Some(format!("{}", arquivos[0].file_stem().unwrap().to_str().unwrap()));
                        //self.nome = format!("{}", arquivos[0].file_stem().unwrap().to_str().unwrap());
                    }else if arquivos.len() == 0{
                        self.username = None;
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
                self.load = false;

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
            self.pagina_anterior = 0;
            return 7;
        }

        0
    }

}

/*
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
}*/
