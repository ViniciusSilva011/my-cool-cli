use rand::Rng;

#[test]
fn run() {
    let mut p = Payment {
        balance: 10000.0,
        status: LoadingStates::Success,
    };

    p.deposit(9999.0);
    p.gamble();
    p.gamble();
    p.pay(67.0);
    p.gamble();
    p.gamble();
}

struct Payment {
    balance: f64,
    status: LoadingStates,
}

impl Payment {
    fn pay(&mut self, value: f64) {
        self.balance -= value;
    }

    fn deposit(&mut self, value: f64) {
        self.balance += value;
    }

    fn gamble(&mut self) -> i64 {
        println!("Gambling...");
        self.status = LoadingStates::Loading;
        let b = self.balance as i64;

        if b <= 0 {
            self.status = LoadingStates::Error;
            return 0;
        }

        let n = rand::thread_rng().gen_range(0..=b);

        let r = b - n;

        let g = rand::thread_rng().gen_bool(0.5);

        match g {
            true => {
                println!("LOST!!");
                self.balance -= r as f64;
            }
            false => {
                println!("WON!!");
                self.balance += r as f64;
            }
        }

        self.status = LoadingStates::Success;
        println!("Current balance: {}", self.balance);
        b - self.balance as i64
    }
}

enum LoadingStates {
    Success,
    Loading,
    Error,
}
