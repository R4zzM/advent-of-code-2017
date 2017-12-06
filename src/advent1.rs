///////////////
/// Private ///
///////////////

const SEQ_1: &[u8; 2040] = b"892195969991735837915273868729548694237967495115412399373194562526947585337233793568278265279199883197167634791293177986152566236718332617536487236879747167999983363832257912445756887314879229925864477761357139855548522513798899853896612387146687716264599943289416326727256525173953861534244979466587895429399159924916364476319573895566795393368411672387263615582128377676293612892723762237191146714286233543514411813323197995953854871628225358543514157867372265718724276911699514971458844849349726276329135118243155698271218844347387457343656446381799296893222256198484465873714311777937421161581798189554141474236239447612421883232173914183732126332838194648583472419154369952477422666389517569944428464617457124369349242479612422673241361777576466946622932243728551273284837934497511114334421486262244982914734452113946361245377351849815584855691778894798219822463298387771923329337634394654439458564233259451453345316753241438267739439225497515276522424441532462541528195782818326918562247278496495764435386667383577543385186827269732261223156824351164841648424564925198783625721396988984481558391866483955533972212164693898955412719161648411279149413443192896864258215498543827458438871355879336892721675937111952479183496982825163456282747678364612135596373533447719867384667516572262124225585623974278833981365494628646614588114147473559138853453189448624976774641922469183942857695986376428944876851497914443873513862319484181787593572987444669767939526294424531262999564948571142342741129862311311313166798363442745792896227642881893134498151552326647933689596516859342242244584714818773791567187322217164347852843751875979415198165627534263527828414549217234322361937785185174993256753483876378332521824515977173397535784236923629636713469151526399149548322849831431526219478653861754364155275865511643923249858589466142474763778413826829226663398467569555747267195129525138917561785436449855933951538973995881954521124753369223898312843734771532342383282987422334196585128526526324291777689689492346231786335851551413876834969878";

fn captcha_solver<F>(seq: &[u8], initializer: F) -> usize 
    where F: Fn(usize) -> usize {

    let mut sum: usize = 0;
    let mut current: usize = 0;
    let mut next: usize = initializer(seq.len());

    while current < seq.len() {
        if next == seq.len() {
            next = 0;
        }

        if seq[current] == seq[next] {
            sum = sum + usize::from(seq[current] - 0x30);
        }

        current += 1;
        next += 1;
    }

    sum
}

//////////////
/// Public ///
//////////////

pub fn first_answer() -> usize {
    captcha_solver(SEQ_1, |_x: usize| 1)
}

pub fn second_answer() -> usize {
    captcha_solver(SEQ_1, |x: usize| x / 2)
}

//////////////////
/// Unit tests ///
//////////////////

#[cfg(test)]
mod tests {

    use advent1::captcha_solver;

    const TEST_SEQ_1A: &[u8; 4] = b"1122";
    const TEST_SEQ_1B : &[u8; 4] = b"1111";
    const TEST_SEQ_1C : &[u8; 4] = b"1234";
    const TEST_SEQ_1D : &[u8; 8] = b"91212129";

    const TEST_SEQ_2A: &[u8; 4] = b"1212";
    const TEST_SEQ_2B : &[u8; 4] = b"1221";
    const TEST_SEQ_2C : &[u8; 6] = b"123425";
    const TEST_SEQ_2D : &[u8; 6] = b"123123";
    const TEST_SEQ_2E : &[u8; 8] = b"12131415";

    #[test]
    fn test_first_captcha() {

        let captcha1 = captcha_solver(TEST_SEQ_1A, |_x: usize| 1);
        assert_eq!(captcha1, 3);

        let captcha2 = captcha_solver(TEST_SEQ_1B, |_x: usize| 1);
        assert_eq!(captcha2, 4);

        let captcha3 = captcha_solver(TEST_SEQ_1C, |_x: usize| 1);
        assert_eq!(captcha3, 0);

        let captcha4 = captcha_solver(TEST_SEQ_1D, |_x: usize| 1);
        assert_eq!(captcha4, 9);
    }

    #[test]
    fn test_second_captcha() {
        let captcha1 = captcha_solver(TEST_SEQ_2A, |x: usize| x / 2);
        assert_eq!(captcha1, 6);

        let captcha2 = captcha_solver(TEST_SEQ_2B, |x: usize| x / 2);
        assert_eq!(captcha2, 0);
        
        let captcha3 = captcha_solver(TEST_SEQ_2C, |x: usize| x / 2);
        assert_eq!(captcha3, 4);
        
        let captcha4 = captcha_solver(TEST_SEQ_2D, |x: usize| x / 2);
        assert_eq!(captcha4, 12);

        let captcha5 = captcha_solver(TEST_SEQ_2E, |x: usize| x / 2);
        assert_eq!(captcha5, 4);
    }
}