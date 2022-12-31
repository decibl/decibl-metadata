use criterion::{black_box, criterion_group, criterion_main, Criterion};
use decibl_metadata::engine::{
    analyticsdb::{
        clear_all_tables, create_all_tables, insert_song_information, populate_database,
    },
    audio_metadata::{add_symphonia_data, AudioFile, AudioFileFLAC, AudioFileMP3},
    config::get_soundfiles_path_1,
};

pub fn bench_symphonia_data(c: &mut Criterion) {
    c.bench_function("add symphonia data", |b| {
        b.iter(|| {
            let filehint = "flac".to_string();
            let filepath = format!("{}/a.flac", get_soundfiles_path_1());
            let data = add_symphonia_data(filepath, filehint);
        })
    });
}

pub fn bench_metaflac_data(c: &mut Criterion) {
    c.bench_function("add metaflac data", |b| {
        b.iter(|| {
            let filehint = "flac".to_string();
            let filepath = format!("{}/a.flac", get_soundfiles_path_1());
            let mut afile = AudioFileFLAC::default();
            afile.add_metaflac_data(filepath);
        })
    });
}

pub fn bench_id3_data(c: &mut Criterion) {
    c.bench_function("add id3 data", |b| {
        b.iter(|| {
            let filepath = format!("{}/cbat.mp3", get_soundfiles_path_1());
            let mut afile = AudioFileMP3::default();
            afile.add_id3_data(filepath);
        })
    });
}

pub fn bench_create_flac(c: &mut Criterion) {
    c.bench_function("insert song", |b| {
        b.iter(|| {
            let filehint = "flac".to_string();
            let filepath = format!("{}/a.flac", get_soundfiles_path_1());
            let mut afile = AudioFileFLAC::default();
            afile.load_file(filepath);
        })
    });
}

pub fn bench_create_mp3(c: &mut Criterion) {
    c.bench_function("insert song", |b| {
        b.iter(|| {
            let filepath = format!("{}/cbat.mp3", get_soundfiles_path_1());
            let mut afile = AudioFileMP3::default();
            afile.load_file(filepath);
        })
    });
}

pub fn bench_insert_song_information(c: &mut Criterion) {
    c.bench_function("insert song", |b| {
        b.iter(|| {
            let filepath = format!("{}/a.flac", get_soundfiles_path_1());
            let mut afile = AudioFileFLAC::default();
            afile.load_file(filepath);

            create_all_tables();
            clear_all_tables();
            insert_song_information(afile);
        })
    });
}

pub fn bench_populate_database(c: &mut Criterion) {
    c.bench_function("insert song", |b| {
        b.iter(|| {
            let filepath = get_soundfiles_path_1();

            create_all_tables();
            clear_all_tables();
            populate_database(filepath);
        })
    });
}

criterion_group!(
    benches,
    bench_symphonia_data,
    bench_metaflac_data,
    bench_id3_data,
    bench_create_flac,
    bench_create_mp3,
    bench_insert_song_information,
    bench_populate_database
);
criterion_main!(benches);
