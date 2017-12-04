use std::io;

fn main() {
    println!("Which day are we running?");
    let mut day_string: String = String::new();
    io::stdin()
        .read_line(&mut day_string)
        .expect("failed to read input.");
    ;
    let day: i32 = day_string.trim().parse().expect("invalid input");

    println!("Number of exercise?");
    let mut exercise_string: String = String::new();
    io::stdin()
        .read_line(&mut exercise_string)
        .expect("failed to read input.");
    ;
    let exercise: i32 = exercise_string.trim().parse().expect("invalid input");

    match day {
        1 => day_one(exercise),
        _ => println!("Day not yet implemented")
    }
}


fn day_one(exercise_number: i32) {
    let puzzle_input: String = String::from("32941994713271959948248321975648598766826\
    38188889768298894243832665654681412886862234525991553276578641\
    26558995917841421838932936149667399161467362634455217941399556\
    22668181383723932139661431249144693976925872511126632178628792\
    33226763533911128893354536353213847122251463857894159819828724\
    82796957643219184778777273288126687546972118933188222814657683\
    29213146382213173932564719985981172896326846633552738459839338\
    45721713497811766995367795857965222183668765517454263354111134\
    84133463134511159613168272619657476316518788933759958334563441\
    34361655397441888661567715856477185551825299366696835816623986\
    18765391487164715724849894563314426959348119286955144439452731\
    76266656874161215325446913172413769983298472893786595671192559\
    26284566171336952595545487193282299386213323251259725471812368\
    12263887375866231118312954369432937359357266467383318326239572\
    87731476512184483112617817398879976521891317882596626881647655\
    97929473599568599892289171362671785717763163452925734898737921\
    49646548747995389669692188457724414468727192819919448275922166\
    32115814136523754522263368837289145184243445852769877434211148\
    24989993838314925776151545912787196567982773773632843794687579\
    98373193231795767644654155432692988651312845433511879457921638\
    93487755757524139436372166723777896245596149355984852258241374\
    82189712124863732327958783629648738559946971496928249171833755\
    45192119453587398199912564474614219929345185468661129966379693\
    81349854247473219817649669474611157692571549396729648725823785\
    41523823655798768943918157598153733191592134755552514887542798\
    88245492373595471189191353244684697662848376529881512529221627\
    31352744122145967278692314516598961122337224114992943624737481\
    84674816419318729725822954259369985351944239165443677995222769\
    14445231582272368388831834437562752119325286474352863554693373\
    71884864956845179775192631561757529538196442684362528281952474\
    71197268721935697856119598967761435399152999682763747129964853\
    67853494734376257511273443736433464496287219615697341973131715\
    166768916149828396454638596713572963686159214116763");
    println!("{}", puzzle_input);

    match exercise_number {
        1 => captcha(puzzle_input),
        2 => captcha_by_size(puzzle_input),
        _ => println!("Exercise not yet implemented")
    }
}

fn captcha(puzzle_input: String) {
    let mut to_sum: Vec<String> = vec![];
    let mut previous_number: String = puzzle_input.chars().rev().take(1).collect();
    for number in puzzle_input.chars() {
        let number_str = number.to_string();

        if previous_number == number_str {
            to_sum.push(number_str.clone())
        }
        previous_number = number_str;
    }
    println!("{:?}", to_sum);

    let converted: Vec<i32> = to_sum.into_iter().map(|x| str_to_i32(x.clone())).collect();
    println!("{:?}", converted);

    let sum: i32 = converted.iter().sum();
    println!("{}", sum);
}

fn captcha_by_size(puzzle_input: String) {
    let doubled_input: String = format!("{}{}", puzzle_input, puzzle_input);
    let look_ahead = puzzle_input.chars().count() / 2;
    let mut to_sum: Vec<String> = vec![];
    for (index, number) in puzzle_input.chars().enumerate() {
        if puzzle_input.chars().nth(index) == doubled_input.chars().nth(index + look_ahead) {
            to_sum.push(number.to_string().clone())
        }
    }
    println!("{:?}", to_sum);

    let converted: Vec<i32> = to_sum.into_iter().map(|x| str_to_i32(x.clone())).collect();
    println!("{:?}", converted);

    let sum: i32 = converted.iter().sum();
    println!("{}", sum);

}

fn day_two(exercise_number: i32) {
    match exercise_number {
        _ => println!("This is where my rust journey ended. I gave up.")
    }
}

fn str_to_i32(str: String) -> i32 {
    let int: i32 = str.trim().parse().expect("invalid number");
    return int
}