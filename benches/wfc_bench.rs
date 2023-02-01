use criterion::{black_box, criterion_group, criterion_main, Criterion, Throughput, BenchmarkId};
use wfc::wfc::{rules, vec3d::Vec3D, baseline, traits::WFC, queueprop, queueprop_bitarrayset, queueprop_bitarrayset_fibheap, stackprop};

fn criterion_benchmark_all(c: &mut Criterion) {
    let mut group = c.benchmark_group("wfc");
    let rules = rules::get_pipes_rules();
    for size in 4..50usize
    {
        group.sample_size(10);
        group.throughput(Throughput::Elements(size.pow(3) as u64));
        let input = Vec3D::with_borders(size, size, size, rules::EMPTY, rules::BORDER);
        if size < 16
        {
            group.bench_with_input(
                BenchmarkId::new("baseline", size),
                &input,
                |b, i| b.iter(|| black_box(
                    baseline::BaseLine::solve(&i, &rules)
            )));
        }
        if size < 35
        {
            group.bench_with_input(
                BenchmarkId::new("queueprop", size),
                &input,
                |b, i| b.iter(|| black_box(
                    queueprop::QueueProp::solve(&i, &rules)
            )));
        }
        group.bench_with_input(
            BenchmarkId::new("queueprop_bitarrayset", size),
            &input,
            |b, i| b.iter(|| black_box(
                queueprop_bitarrayset::QueuePropBitArraySet::solve(&i, &rules)
        )));
        group.bench_with_input(
            BenchmarkId::new("queueprop_bitarrayset_fibheap", size),
            &input,
            |b, i| b.iter(|| black_box(
                queueprop_bitarrayset_fibheap::QueuePropBitArraySetFibHeap::solve(&i, &rules)
        )));
    }
    group.finish();
}

// fn criterion_benchmark_queue_stack(c: &mut Criterion) {
//     let mut group = c.benchmark_group("queueVsStack");
//     let rules = rules::get_pipes_rules();
//     for size in 15..33
//     {
//         group.sample_size(usize::max(100/size, 10));
//         group.throughput(Throughput::Elements(size.pow(3) as u64));
//         let input = Vec3D::with_borders(size, size, size, rules::EMPTY, rules::BORDER);
//         group.bench_with_input(
//             BenchmarkId::new("queueprop", size),
//             &input,
//             |b, i| b.iter(|| black_box(
//                 queueprop::QueueProp::solve(&i, &rules)
//         )));
//         group.bench_with_input(
//             BenchmarkId::new("stackprop", size),
//             &input,
//             |b, i| b.iter(|| black_box(
//                 stackprop::StackProp::solve(&i, &rules)
//         )));
//     }
//     group.finish();
// }

// fn criterion_benchmark_fibheap(c: &mut Criterion) {
//     let mut group = c.benchmark_group("fibheap_only");
//     let rules = rules::get_pipes_rules();
//     let size = 40;
//     let input = Vec3D::with_borders(size, size, size, rules::EMPTY, rules::BORDER);
//     group.sample_size(10);
//     group.throughput(Throughput::Elements(size.pow(3) as u64));
//     group.bench_with_input(
//         BenchmarkId::new("fibheap", size),
//         &input,
//         |b, i| b.iter(|| black_box(
//             queueprop_bitarrayset_fibheap::QueuePropBitArraySetFibHeap::solve(&i, &rules)
//     )));
//     group.finish();
// }

// fn criterion_benchmark_bitarray(c: &mut Criterion) {
//     let mut group = c.benchmark_group("bitarray_only");
//     let rules = rules::get_pipes_rules();
//     let size = 34;
//     let input = Vec3D::with_borders(size, size, size, rules::EMPTY, rules::BORDER);
//     group.sample_size(10);
//     group.throughput(Throughput::Elements(size.pow(3) as u64));
//     group.bench_with_input(
//         BenchmarkId::new("bitarray", size),
//         &input,
//         |b, i| b.iter(|| black_box(
//             queueprop_bitarrayset::QueuePropBitArraySet::solve(&i, &rules)
//     )));
//     group.finish();
// }

criterion_group!(benches, criterion_benchmark_all);
criterion_main!(benches);