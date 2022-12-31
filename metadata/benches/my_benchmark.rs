use criterion::{black_box, criterion_group, criterion_main, Criterion};
use decibl_metadata::engine::{config::get_soundfiles_path_1, audio_metadata::{add_symphonia_data, AudioFileFLAC}};

pub fn bench_symphonia_data(c: &mut Criterion) {
    c.bench_function("add symphonia data", |b| b.iter(|| {
        let filehint = "flac".to_string();
        let filepath = format!("{}/a.flac", get_soundfiles_path_1());
        let data = add_symphonia_data(filepath, filehint);
    }));
}

pub fn bench_metaflac_data(c: &mut Criterion) {
    c.bench_function("add metaflac data", |b| b.iter(|| {
        let filehint = "flac".to_string();
        let filepath = format!("{}/a.flac", get_soundfiles_path_1());
        let mut afile = AudioFileFLAC::default();
        afile.add_metaflac_data(filepath);
    }));
}
criterion_group!(benches, bench_symphonia_data);
criterion_main!(benches);