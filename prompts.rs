use std::io::{self, Write};

// A utility function to receive answers.

fn prompt() -> bool {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    return input.to_ascii_lowercase().starts_with("y");
}

// pub stands for "public", not pubs!
// "public" allows functions to be
// seen by other project files.
pub fn intro() -> () {
    println!(
        "
	==================================
			   STICKER QUEST
	==================================

	Huh, there's a vibration coming from
	your pocket, and you wonder what causes
	it.

	It's your phone. Obviously.
	It's vibrating in your pocket.

	You pull out the phone out of your
	pocket and pick it up instinctively.

	'I need you for a job.', says the person
	on the other end of the line.

	'Someone broke into our headquarters and
	seems to have locked themselves in the
	room where we package all the stickers!
	If you happen to be nearby, could you please
	check what's up?'. The caller sounds desperate.

	You're trying to make sense of why this
	should matter to you, but it seems like
	you're the only person who can deal
	with this.

	Do you accept?"
    );

    // constant variables are great for hardcoding datas
    // const enw mporei na allaksei se allo function?
    // mhpws xanw kati apo scope?
    const LOSS_MSG: &str = "
		You hang up on the caller. A few hours later,
		you realize that your sticker order got cancelled.
		
		... Someone apparently stole them. Gosh, this 
		wouldn't have happened if you didn't decide to
		not do anything! I can only imagine the same
		you must be feeling right now.";

    if prompt() {
        how_to_unlock();
    } else {
        lose(LOSS_MSG);
    }

    // While the previous way is syntactically correct,
    // match patterns are used more often in Rust.
    // However, this has been commented out in order to
    // keep things beginner-friendly, under the assumption
    // that most people who plan to look into this possess
    // some experience with programming.
    /*
    match prompt() {
        true => how_to_unlock(),
        false => lose(),
    }
    */
}

pub fn how_to_unlock() {
    println!(
        "
	-----------------------------
	|            Hm...          |
	|                           |
	|        ____________       |
	|       |            |      |
	|       |            |      |
	|       |            |      |
	|       |            | ||   |
	|       |         O  | ||   |
	|       |            |      |
	|       |            |      |
	|       |            |      |
	-----------------------------

	The door's locked. On the other side,
	you can hear someone typing frantically
	as if their life depended on it.

	Oh, you just heard a squeak! It sounds
	vaguely familiar. \"Could it be?\",
	you ask yourself.

	There's a keycard lock next to the door,
	but using the card you have on you may alert
	the intruder. Alternatively, you could just
	try to act fast and bruteforce your way
	through. That'd be a bit loud, though.
	
	Do you use your card?"
    );

    const LOSS_MSG: &str = "
		You scanned your keycard anyway...
		and the lock rejected it. The intruder
		noticed and just squeaked again,
		completely unbothered. They know
		that you want to foil the plans,
		so they swiftly pushed a bunch of heavy
		objects in front of the door, before you
		even managed to consider the alternative
		option.
		
		The door got unlocked shortly afterwards,
		with both the intruder and the stickers
		having completely vanished, probably through
		the backdoor.";

    if prompt() {
        face_intruder();
    } else {
        lose(LOSS_MSG);
    }
}

pub fn face_intruder() {
    println!(
        "
		-----------------------------
		|                           |
		|                           |
		|          _________        |
		|         /          \\      |
		|        /        .   |     |
		|       /          \\_ |     |
		|      /      ________/     |
		|     /     /               |
		|    /     /     - Huh?     |
		|   /     /                 |
		|  /     /                  |
		-----------------------------

		\"... Orpheus?\" you say, sounding horrified.
		\"What are you doing here?\", you ask.

		\"hrm!\", she replies joyfully.

		\"..... uh, could you not?\"

		Orpheus shrugs.

		\"hrum\"

		You hand over one of the sticker to Orpheus and
		deescalate  the situation by convincing her that
		using the same stickers all over the lid of her
		laptop just won't look aesthetically pleasing
		enough.
		
		Great job, you saved the day!
	"
    );
}

pub fn lose(msg: &str) {
    println!(
        "{}",
        format!(
            "\n
	-----------------------------
	|           OH NO!          |
	|                           |
	|    ____________________   |
	|    |                  |   |
	|    |    |       |     |   |
	|    |                  |   |
	|    |    ___________   |   |
	|    |   /            \\ |   |
	|    |                  |   |
	|    |__________________|   |
	|                           |
	-----------------------------
	
	{}
	
	
	GAME. OVER.\n",
            msg
        )
    );

    return;
}
