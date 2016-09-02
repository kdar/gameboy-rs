use std::sync::mpsc::{self, Sender, Receiver};
use std::time::Instant;

use glium::{glutin, Rect, DisplayBuild, BlitTarget, Surface, texture, uniforms};
use glium::backend::glutin_backend::GlutinFacade;
use glium::glutin::{ElementState, Event, MouseButton, MouseScrollDelta, VirtualKeyCode, TouchPhase};
use imgui::{ImGui, Ui, ImGuiSetCond_FirstUseEver, ImVec4};
use imgui::glium_renderer::Renderer;
use time::{Duration, SteadyTime};

use super::super::GbEvent;
use super::super::video;
use super::super::gamepad::Button as GButton;

const CLEAR_COLOR: (f32, f32, f32, f32) = (1.0, 1.0, 1.0, 1.0);

pub enum ImGuiCol {
  Text,
  TextDisabled,
  WindowBg,
  ChildWindowBg,
  PopupBg,
  Border,
  BorderShadow,
  FrameBg,
  FrameBgHovered,
  FrameBgActive,
  TitleBg,
  TitleBgCollapsed,
  TitleBgActive,
  MenuBarBg,
  ScrollbarBg,
  ScrollbarGrab,
  ScrollbarGrabHovered,
  ScrollbarGrabActive,
  ComboBg,
  CheckMark,
  SliderGrab,
  SliderGrabActive,
  Button,
  ButtonHovered,
  ButtonActive,
  Header,
  HeaderHovered,
  HeaderActive,
  Column,
  ColumnHovered,
  ColumnActive,
  ResizeGrip,
  ResizeGripHovered,
  ResizeGripActive,
  CloseButton,
  CloseButtonHovered,
  CloseButtonActive,
  PlotLines,
  PlotLinesHovered,
  PlotHistogram,
  PlotHistogramHovered,
  TextSelectedBg,
  ModalWindowDarkening,
}

pub struct GliumUi {
  display: Option<GlutinFacade>,
  imgui: Option<ImGui>,
  renderer: Option<Renderer>,
  last_frame: Instant,
  event_sender: Sender<GbEvent>,
  frame_receiver: Receiver<Vec<[u8; 4]>>,
  mouse_pos: (i32, i32),
  mouse_pressed: (bool, bool, bool),
  mouse_wheel: f32,
}

impl Default for GliumUi {
  fn default() -> GliumUi {
    let (_, null_receiver) = mpsc::channel();
    let (null_sender, _) = mpsc::channel();
    GliumUi {
      display: None,
      imgui: None,
      renderer: None,
      last_frame: Instant::now(),
      event_sender: null_sender,
      frame_receiver: null_receiver,
      mouse_pos: (0, 0),
      mouse_pressed: (false, false, false),
      mouse_wheel: 0.0,
    }
  }
}

impl GliumUi {
  pub fn new(s: Sender<GbEvent>, r: Receiver<Vec<[u8; 4]>>) -> GliumUi {
    let display = glutin::WindowBuilder::new()
      .build_glium()
      .unwrap();
    let mut imgui = ImGui::init();
    let renderer = Renderer::init(&mut imgui, &display).unwrap();

    {
      let mut style = imgui.style_mut();

      style.colors[ImGuiCol::Text as usize] = ImVec4::new(0.00, 0.00, 0.00, 1.00);
      style.colors[ImGuiCol::TextDisabled as usize] = ImVec4::new(0.60, 0.60, 0.60, 1.00);
      style.colors[ImGuiCol::PopupBg as usize] = ImVec4::new(0.94, 0.94, 0.94, 1.00);
      style.colors[ImGuiCol::WindowBg as usize] = ImVec4::new(0.94, 0.94, 0.94, 1.00);
      style.colors[ImGuiCol::ChildWindowBg as usize] = ImVec4::new(0.00, 0.00, 0.00, 0.00);
      style.colors[ImGuiCol::Border as usize] = ImVec4::new(0.00, 0.00, 0.00, 0.39);
      style.colors[ImGuiCol::BorderShadow as usize] = ImVec4::new(1.00, 1.00, 1.00, 0.10);
      style.colors[ImGuiCol::FrameBg as usize] = ImVec4::new(1.00, 1.00, 1.00, 1.00);
      style.colors[ImGuiCol::FrameBgHovered as usize] = ImVec4::new(0.26, 0.59, 0.98, 0.40);
      style.colors[ImGuiCol::FrameBgActive as usize] = ImVec4::new(0.26, 0.59, 0.98, 0.67);
      style.colors[ImGuiCol::TitleBg as usize] = ImVec4::new(0.89, 0.89, 0.89, 1.00);
      style.colors[ImGuiCol::TitleBgCollapsed as usize] = ImVec4::new(0.89, 0.89, 0.89, 0.51);
      style.colors[ImGuiCol::TitleBgActive as usize] = ImVec4::new(0.82, 0.82, 0.82, 1.00);
      style.colors[ImGuiCol::MenuBarBg as usize] = ImVec4::new(0.86, 0.86, 0.86, 1.00);
      style.colors[ImGuiCol::ScrollbarBg as usize] = ImVec4::new(0.98, 0.98, 0.98, 0.53);
      style.colors[ImGuiCol::ScrollbarGrab as usize] = ImVec4::new(0.69, 0.69, 0.69, 0.80);
      style.colors[ImGuiCol::ScrollbarGrabHovered as usize] = ImVec4::new(0.49, 0.49, 0.49, 0.80);
      style.colors[ImGuiCol::ScrollbarGrabActive as usize] = ImVec4::new(0.49, 0.49, 0.49, 1.00);
      style.colors[ImGuiCol::ComboBg as usize] = ImVec4::new(0.86, 0.86, 0.86, 0.99);
      style.colors[ImGuiCol::CheckMark as usize] = ImVec4::new(0.26, 0.59, 0.98, 1.00);
      style.colors[ImGuiCol::SliderGrab as usize] = ImVec4::new(0.26, 0.59, 0.98, 0.78);
      style.colors[ImGuiCol::SliderGrabActive as usize] = ImVec4::new(0.26, 0.59, 0.98, 1.00);
      style.colors[ImGuiCol::Button as usize] = ImVec4::new(0.26, 0.59, 0.98, 0.40);
      style.colors[ImGuiCol::ButtonHovered as usize] = ImVec4::new(0.26, 0.59, 0.98, 1.00);
      style.colors[ImGuiCol::ButtonActive as usize] = ImVec4::new(0.06, 0.53, 0.98, 1.00);
      style.colors[ImGuiCol::Header as usize] = ImVec4::new(0.26, 0.59, 0.98, 0.31);
      style.colors[ImGuiCol::HeaderHovered as usize] = ImVec4::new(0.26, 0.59, 0.98, 0.80);
      style.colors[ImGuiCol::HeaderActive as usize] = ImVec4::new(0.26, 0.59, 0.98, 1.00);
      style.colors[ImGuiCol::Column as usize] = ImVec4::new(0.39, 0.39, 0.39, 1.00);
      style.colors[ImGuiCol::ColumnHovered as usize] = ImVec4::new(0.26, 0.59, 0.98, 0.78);
      style.colors[ImGuiCol::ColumnActive as usize] = ImVec4::new(0.26, 0.59, 0.98, 1.00);
      style.colors[ImGuiCol::ResizeGrip as usize] = ImVec4::new(1.00, 1.00, 1.00, 0.00);
      style.colors[ImGuiCol::ResizeGripHovered as usize] = ImVec4::new(0.26, 0.59, 0.98, 0.67);
      style.colors[ImGuiCol::ResizeGripActive as usize] = ImVec4::new(0.26, 0.59, 0.98, 0.95);
      style.colors[ImGuiCol::CloseButton as usize] = ImVec4::new(0.59, 0.59, 0.59, 0.50);
      style.colors[ImGuiCol::CloseButtonHovered as usize] = ImVec4::new(0.98, 0.39, 0.36, 1.00);
      style.colors[ImGuiCol::CloseButtonActive as usize] = ImVec4::new(0.98, 0.39, 0.36, 1.00);
      style.colors[ImGuiCol::PlotLines as usize] = ImVec4::new(0.39, 0.39, 0.39, 1.00);
      style.colors[ImGuiCol::PlotLinesHovered as usize] = ImVec4::new(1.00, 0.43, 0.35, 1.00);
      style.colors[ImGuiCol::PlotHistogram as usize] = ImVec4::new(0.90, 0.70, 0.00, 1.00);
      style.colors[ImGuiCol::PlotHistogramHovered as usize] = ImVec4::new(1.00, 0.60, 0.00, 1.00);
      style.colors[ImGuiCol::TextSelectedBg as usize] = ImVec4::new(0.26, 0.59, 0.98, 0.35);
      style.colors[ImGuiCol::ModalWindowDarkening as usize] = ImVec4::new(0.20, 0.20, 0.20, 0.35);
    }

    GliumUi {
      display: Some(display),
      imgui: Some(imgui),
      renderer: Some(renderer),
      event_sender: s,
      frame_receiver: r,
      ..GliumUi::default()
    }
  }

  pub fn run(&mut self) {
    let mut frame_count = 0;
    let mut start = SteadyTime::now();

    let mut texture = {
      let display = self.display.as_mut().unwrap();
      texture::texture2d::Texture2d::empty_with_format(display,
                                                       texture::UncompressedFloatFormat::U8U8U8,
                                                       texture::MipmapsOption::NoMipmap,
                                                       video::SCREEN_WIDTH as u32,
                                                       video::SCREEN_HEIGHT as u32)
        .unwrap()
    };

    loop {
      self.render(&mut texture, hello_world);
      frame_count += 1;

      if SteadyTime::now() - start >= Duration::seconds(1) {
        println!("Gameboy-rs: {} fps", frame_count);
        frame_count = 0;
        start = SteadyTime::now();
      }

      let active = self.update_events();
      if !active {
        break;
      }
    }
  }

  fn update_mouse(&mut self) {
    let imgui = self.imgui.as_mut().unwrap();
    let scale = imgui.display_framebuffer_scale();
    imgui.set_mouse_pos(self.mouse_pos.0 as f32 / scale.0,
                        self.mouse_pos.1 as f32 / scale.1);
    imgui.set_mouse_down(&[self.mouse_pressed.0,
                           self.mouse_pressed.1,
                           self.mouse_pressed.2,
                           false,
                           false]);
    imgui.set_mouse_wheel(self.mouse_wheel / scale.1);
    self.mouse_wheel = 0.0;
  }

  fn update_events(&mut self) -> bool {
    let display = self.display.as_mut().unwrap();
    let imgui = self.imgui.as_mut().unwrap();

    for event in display.poll_events() {
      match event {
        Event::Closed => return false,
        Event::KeyboardInput(state, _, code) => {
          let pressed = state == ElementState::Pressed;
          match code {
            Some(VirtualKeyCode::Left) |
            Some(VirtualKeyCode::A) => {
              self.event_sender.send(GbEvent::Button(GButton::Left, pressed)).unwrap()
            }
            Some(VirtualKeyCode::Right) |
            Some(VirtualKeyCode::D) => {
              self.event_sender.send(GbEvent::Button(GButton::Right, pressed)).unwrap()
            }
            Some(VirtualKeyCode::Up) |
            Some(VirtualKeyCode::W) => {
              self.event_sender.send(GbEvent::Button(GButton::Up, pressed)).unwrap()
            }
            Some(VirtualKeyCode::Down) |
            Some(VirtualKeyCode::S) => {
              self.event_sender.send(GbEvent::Button(GButton::Down, pressed)).unwrap()
            }
            // Some(VirtualKeyCode::LControl) |
            // Some(VirtualKeyCode::RControl) => imgui.set_key_ctrl(pressed),
            // Some(VirtualKeyCode::LShift) |
            // Some(VirtualKeyCode::RShift) => imgui.set_key_shift(pressed),
            // Some(VirtualKeyCode::LAlt) |
            // Some(VirtualKeyCode::RAlt) => imgui.set_key_alt(pressed),
            // Some(VirtualKeyCode::LWin) |
            // Some(VirtualKeyCode::RWin) => imgui.set_key_super(pressed),
            _ => {}
          }
        }
        Event::MouseMoved(x, y) => self.mouse_pos = (x, y),
        Event::MouseInput(state, MouseButton::Left) => {
          self.mouse_pressed.0 = state == ElementState::Pressed
        }
        Event::MouseInput(state, MouseButton::Right) => {
          self.mouse_pressed.1 = state == ElementState::Pressed
        }
        Event::MouseInput(state, MouseButton::Middle) => {
          self.mouse_pressed.2 = state == ElementState::Pressed
        }
        Event::MouseWheel(MouseScrollDelta::LineDelta(_, y), TouchPhase::Moved) |
        Event::MouseWheel(MouseScrollDelta::PixelDelta(_, y), TouchPhase::Moved) => {
          self.mouse_wheel = y
        }
        Event::ReceivedCharacter(c) => imgui.add_input_character(c),
        _ => (),
      }
    }
    true
  }

  fn render<F: FnMut(&Ui)>(&mut self, texture: &mut texture::texture2d::Texture2d, mut run_ui: F) {
    let mut data = vec![];
    if let Ok(d) = self.frame_receiver.try_recv() {
      data = d;
    }

    let now = Instant::now();
    let delta = now - self.last_frame;
    let delta_s = delta.as_secs() as f32 + delta.subsec_nanos() as f32 / 1_000_000_000.0;
    self.last_frame = now;

    self.update_mouse();

    if data.is_empty() {
      return;
    }

    let display = self.display.as_mut().unwrap();
    let imgui = self.imgui.as_mut().unwrap();
    let renderer = self.renderer.as_mut().unwrap();

    let mut target = display.draw();
    target.clear_color(CLEAR_COLOR.0, CLEAR_COLOR.1, CLEAR_COLOR.2, CLEAR_COLOR.3);

    use std;
    let tmp: Vec<u8> = data.iter()
      .flat_map(|v| v.iter().map(|&r| r))
      .collect();
    let rawimage2d = texture::RawImage2d {
      data: std::borrow::Cow::Borrowed(tmp.as_slice()),
      width: video::SCREEN_WIDTH as u32,
      height: video::SCREEN_HEIGHT as u32,
      format: texture::ClientFormat::U8U8U8U8,
    };
    texture.write(Rect {
                    left: 0,
                    bottom: 0,
                    width: video::SCREEN_WIDTH as u32,
                    height: video::SCREEN_HEIGHT as u32,
                  },
                  rawimage2d);

    let (target_w, target_h) = target.get_dimensions();
    texture.as_surface().blit_whole_color_to(&target,
                                             &BlitTarget {
                                               left: 0,
                                               bottom: target_h,
                                               width: target_w as i32,
                                               height: -(target_h as i32),
                                             },
                                             uniforms::MagnifySamplerFilter::Nearest);




    // let mut target = display.draw();


    let window = display.get_window().unwrap();
    let size_points = window.get_inner_size_points().unwrap();
    let size_pixels = window.get_inner_size_pixels().unwrap();

    // let ui = imgui.frame(size_points, size_pixels, delta_s);
    //
    // run_ui(&ui);
    //
    // renderer.render(&mut target, ui).unwrap();

    target.finish().unwrap();
  }
}

fn hello_world(ui: &Ui) {
  ui.show_default_style_editor();
  ui.window(im_str!("Hello world"))
    .size((300.0, 100.0), ImGuiSetCond_FirstUseEver)
    .build(|| {
      ui.text(im_str!("Hello world!"));
      ui.text(im_str!("This...is...imgui-rs!"));
      ui.separator();
      let mouse_pos = ui.imgui().mouse_pos();
      ui.text(im_str!("Mouse Position: ({:.1},{:.1})", mouse_pos.0, mouse_pos.1));
    })
}
