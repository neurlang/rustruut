use std::time::Instant;

use rustruut::models::requests::PhonemizeSentence;
use rustruut::{DependencyInjection, Phonemizer};

const NUM_REQUESTS: u32 = 1000;
const LANGUAGE: &str = "English";
const SENTENCE: &str = "hello world";

#[test]
fn stress_test_1000_requests() {
    let di: DependencyInjection = DependencyInjection::new();
    let phonemizer = Phonemizer::new(di);

    let mut min_latency = f64::MAX;
    let mut max_latency = f64::MIN;
    let mut total_latency = 0.0;
    let mut errors: Vec<usize> = Vec::new();
    let mut total_words: Vec<usize> = Vec::new();

    let overall_start = Instant::now();

    for i in 0..NUM_REQUESTS as usize {
        let req = PhonemizeSentence {
            ipa_flavors: vec![],
            language: LANGUAGE.to_string(),
            languages: vec![],
            sentence: SENTENCE.to_string(),
            is_reverse: false,
            split_sentences: false,
        };

        let start = Instant::now();
        let result = phonemizer.sentence(req);
        let elapsed = start.elapsed().as_secs_f64();

        total_latency += elapsed;
        if elapsed < min_latency {
            min_latency = elapsed;
        }
        if elapsed > max_latency {
            max_latency = elapsed;
        }

        match result {
            Ok(resp) => {
                let word_count = resp.words.len();
                total_words.push(word_count);
                if word_count != 2 {
                    errors.push(i);
                }
                if word_count > 0 && resp.words[0].clean_word != "hello" {
                    errors.push(i);
                }
                if word_count > 1 && resp.words[1].clean_word != "world" {
                    errors.push(i);
                }
            }
            Err(e) => {
                errors.push(i);
                eprintln!("Request {} failed: {}", i, e);
            }
        }
    }

    let overall_elapsed = overall_start.elapsed();

    let avg_latency = total_latency / NUM_REQUESTS as f64;
    let throughput = NUM_REQUESTS as f64 / overall_elapsed.as_secs_f64();

    println!("\n=== Stress Test Results ===");
    println!("Requests:     {}", NUM_REQUESTS);
    println!("Total time:   {:.3}s", overall_elapsed.as_secs_f64());
    println!("Throughput:   {:.0} req/s", throughput);
    println!("Avg latency:  {:.3}s ({:.1}ms)", avg_latency, avg_latency * 1000.0);
    println!("Min latency:  {:.3}s ({:.1}ms)", min_latency, min_latency * 1000.0);
    println!("Max latency:  {:.3}s ({:.1}ms)", max_latency, max_latency * 1000.0);
    println!("Errors:       {}/{}", errors.len(), NUM_REQUESTS);
    println!("==========================\n");

    assert!(
        errors.is_empty(),
        "{} requests failed (indices: {:?})",
        errors.len(),
        errors
    );
}
