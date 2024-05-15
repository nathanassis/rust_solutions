use std::io;

fn read_input() -> Vec<f32> {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("");
    let input: Vec<&str> = input.split_whitespace().collect();

    input
        .into_iter()
        .map(|x| x.trim().parse::<f32>().unwrap())
        .collect()
}

fn main() {
    let notes = read_input();
    let x = vec![2.0, 3.0, 4.0, 1.0];

    let mut media = 0.0;
    for i in 0..x.len() {
        media += notes[i] * x[i];
    }

    media /= 10.0;
    if media >= 7.0 {
        println!("Media: {:.1}\nAluno aprovado.", media);
    } else if media < 5.0 {
        println!("Media: {:.1}\nAluno reprovado.", media);
    } else {
        let mut exam_note = String::new();
        io::stdin().read_line(&mut exam_note).expect("");
        let exam_note = exam_note.trim().parse::<f32>().unwrap();

        println!(
            "Media: {:.1}\nAluno em exame.\nNota do exame: {:.1}\n{}\nMedia final: {:.1}",
            media, exam_note, if (exam_note + media) / 2.0 >= 5.0 { "Aluno aprovado." } else { "Aluno reprovado." }, (exam_note + media) / 2.0
        );
    }
}
