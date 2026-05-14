#[allow(rustdoc::missing_crate_level_docs)]
use eframe::egui;
use rodio::{source::SineWave, OutputStream, Sink, Source};
use std::fs::OpenOptions;
use std::io::Write;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

struct MetronomeApp {
    bpm: u32,
    beats_per_bar: u32,
    is_running: bool,
    count: u32,
    total_practice_seconds: u64,
    start_time: Option<Instant>,
    last_practice_duration: u64,
    // スレッド間共有のためのフラグ
    running_state: Arc<Mutex<bool>>,
}

impl Default for MetronomeApp {
    fn default() -> Self {
        Self {
            bpm: 120,
            beats_per_bar: 4,
            is_running: false,
            count: 0,
            total_practice_seconds: 0,
            start_time: None,
            last_practice_duration: 0,
            running_state: Arc::new(Mutex::new(false)),
        }
    }
}

impl MetronomeApp {
    fn save_log(&self, seconds: u64) {
        let now = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
        let log_entry = format!(
            "[{}] BPM:{}, 拍子:{}, 時間:{}秒\n",
            now, self.bpm, self.beats_per_bar, seconds
        );

        if let Ok(mut file) = OpenOptions::new()
            .create(true)
            .append(true)
            .open("practice_log.txt")
        {
            let _ = file.write_all(log_entry.as_bytes());
        }
    }

    fn spawn_metronome_thread(&self) {
        let bpm = self.bpm;
        let beats_per_bar = self.beats_per_bar;
        let running = Arc::clone(&self.running_state);

        std::thread::spawn(move || {
            let (_stream, stream_handle) = OutputStream::try_default().unwrap();
            let sink = Sink::try_new(&stream_handle).unwrap();

            let mut local_count = 0;
            loop {
                if !*running.lock().unwrap() {
                    break;
                }

                let is_first_beat = (local_count % beats_per_bar) == 0;
                let freq = if is_first_beat { 880.0 } else { 440.0 };
                
                // ビープ音の生成
                let source = SineWave::new(freq)
                    .take_duration(Duration::from_millis(50))
                    .amplify(0.2);
                sink.append(source);

                let interval = Duration::from_secs_f64(60.0 / bpm as f64);
                std::thread::sleep(interval);
                local_count += 1;
            }
        });
    }
}

impl eframe::App for MetronomeApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.heading("Metronome with Log (Rust)");
                ui.add_space(10.0);

                // BPM表示とスライダー
                ui.label(format!("BPM: {}", self.bpm));
                ui.add(egui::Slider::new(&mut self.bpm, 40..=240));

                // 拍子設定
                ui.horizontal(|ui| {
                    ui.label("拍子:");
                    egui::ComboBox::from_label("")
                        .selected_text(format!("{}", self.beats_per_bar))
                        .show_ui(ui, |ui| {
                            ui.selectable_value(&mut self.beats_per_bar, 2, "2");
                            ui.selectable_value(&mut self.beats_per_bar, 3, "3");
                            ui.selectable_value(&mut self.beats_per_bar, 4, "4");
                            ui.selectable_value(&mut self.beats_per_bar, 6, "6");
                        });
                });

                ui.add_space(10.0);

                // 開始・停止ボタン
                let button_text = if self.is_running { "STOP" } else { "START" };
                if ui.button(button_text).clicked() {
                    self.is_running = !self.is_running;
                    *self.running_state.lock().unwrap() = self.is_running;

                    if self.is_running {
                        self.start_time = Some(Instant::now());
                        self.spawn_metronome_thread();
                    } else {
                        if let Some(start) = self.start_time {
                            let elapsed = start.elapsed().as_secs();
                            self.last_practice_duration = elapsed;
                            self.total_practice_seconds += elapsed;
                            self.save_log(elapsed);
                        }
                    }
                }

                ui.add_space(20.0);
                ui.separator();

                // ログ表示
                ui.label(format!(
                    "今回の練習時間: {}秒 (合計: {}秒)",
                    self.last_practice_duration, self.total_practice_seconds
                ));

                if ui.button("ログファイル（直近）を表示").clicked() {
                    // 簡易的にコンソール表示、またはOSの機能でファイルを開く
                    let _ = std::process::Command::new("notepad.exe")
                        .arg("practice_log.txt")
                        .spawn();
                }
            });
        });

        // 画面の再描画を促す（インジケータなどを作る場合用）
        ctx.request_repaint_after(Duration::from_millis(100));
    }
}

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        // viewport 内でサイズを指定するように修正
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([350.0, 400.0]),
        ..Default::default()
    };
    
    eframe::run_native(
        "Rust Metronome",
        options,
        Box::new(|_cc| Box::new(MetronomeApp::default())),
    )
}