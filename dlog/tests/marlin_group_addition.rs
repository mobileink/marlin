/*********************************************************************************************************

This source file tests constraints for the Weierstrass curve y^2 = x^3 + 14 group addition
of non-special pairs of points

    (x2 - x1) * s = y2 - y1
    s * s = x1 + x2 + x3
    (x1 - x3) * s = y3 + y1

    For the variable labeling
    [1, x1, x2, x3, y1, y2, y3, s]

    the Index constraints are

    a=[[0,-1,1,0,0,0,0,0], [0,0,0,0,0,0,0,1], [0,1,0,-1,0,0,0,0]]
    b=[[0,0,0, 0,0,0,0,1], [0,0,0,0,0,0,0,1], [0,0,0,0,0,0,0,1]]
    c=[[0,0,0,0,-1,1,0,0], [0,1,1,1,0,0,0,0], [0,0,0,0,1,0,1,0]]

    The test verifies both positive and negative outcomes for satisfying and not satisfying witnesses


**********************************************************************************************************/

use marlin_protocol_dlog::index::{SRSSpec, Index};
use sprs::{CsMat, CsVecView};
use algebra::{UniformRand, bn_382::g::{Affine, Bn_382GParameters}, AffineCurve, One, Zero};
use marlin_protocol_dlog::{prover::{ProverProof}};
use commitment_dlog::{srs::SRS, commitment::{CommitmentCurve, ceil_log2, b_poly_coefficients}};
use oracle::{sponge::{DefaultFrSponge, DefaultFqSponge}, poseidon::{ArithmeticSpongeParams, MarlinSpongeConstants as SC}};
use rand_core::{RngCore, OsRng};
use std::{io, io::Write};
use std::time::Instant;
use colored::Colorize;
use ff_fft::{DensePolynomial};
use groupmap::GroupMap;

type Fr = <Affine as AffineCurve>::ScalarField;
const MAX_SIZE: usize = 8;

#[test]
fn dlog_marlin_group_addition()
{
    let rng = &mut OsRng;

    // field unity element
    let one = Fr::one();
    // field negative unit element
    let neg1 = -one;

    // our circuit cinstraint system

    let mut a = CsMat::<Fr>::zero((5, 8));
    let mut b = CsMat::<Fr>::zero((5, 8));
    let mut c = CsMat::<Fr>::zero((5, 8));

    a = a
    .append_outer_csvec(CsVecView::<Fr>::new_view(8, &[1, 2], &[neg1, one]).unwrap())
    .append_outer_csvec(CsVecView::<Fr>::new_view(8, &[7], &[one]).unwrap())
    .append_outer_csvec(CsVecView::<Fr>::new_view(8, &[1, 3], &[one, neg1]).unwrap());

    b = b
    .append_outer_csvec(CsVecView::<Fr>::new_view(8, &[7], &[one]).unwrap())
    .append_outer_csvec(CsVecView::<Fr>::new_view(8, &[7], &[one]).unwrap())
    .append_outer_csvec(CsVecView::<Fr>::new_view(8, &[7], &[one]).unwrap());

    c = c
    .append_outer_csvec(CsVecView::<Fr>::new_view(8, &[4, 5], &[neg1, one]).unwrap())
    .append_outer_csvec(CsVecView::<Fr>::new_view(8, &[1, 2, 3], &[one, one, one]).unwrap())
    .append_outer_csvec(CsVecView::<Fr>::new_view(8, &[4, 6], &[one, one]).unwrap());

    let srs = SRS::create(MAX_SIZE);

    let index = Index::<Affine>::create
    (
        a,
        b,
        c,
        4,
        MAX_SIZE,
        oracle::bn_382::fq::params() as ArithmeticSpongeParams<Fr>,
        oracle::bn_382::fp::params(),
        SRSSpec::Use(&srs)
    ).unwrap();

    positive(&index, rng);
    negative(&index);
}

fn positive(index: &Index<Affine>, rng: &mut dyn RngCore)
where <Fr as std::str::FromStr>::Err : std::fmt::Debug
{
    let mut points = Vec::<(Fr, Fr, Fr, Fr, Fr, Fr)>::new();
    points.push
    ((
        <Fr as std::str::FromStr>::from_str("1580733493061982224102642506998085489258052950031005050616926032148684443068721819617638109822422025817760865738650").unwrap(),
        <Fr as std::str::FromStr>::from_str("2120085809980346347658418912345228674556840189092324973615155047510076539377582421094477427199660756057892003266260").unwrap(),
        <Fr as std::str::FromStr>::from_str("2931063856920074489991213592706123181795217105777923458970198160424184864319820345938320384765820615002087379202625").unwrap(),
        <Fr as std::str::FromStr>::from_str("3634752862255786778633521512827318855463765750440270000121873025280392646700033626519512004314174921695952488907036").unwrap(),
        <Fr as std::str::FromStr>::from_str("1294507634713475436209031771946300666248735314827817267504772563137113162405833758696084205208524338669398158984830").unwrap(),
        <Fr as std::str::FromStr>::from_str("114798453479363569901779346943141343003503211376947251274646193677028801959107629567000376881703165185002804693406").unwrap(),
    ));
    points.push
    ((
        <Fr as std::str::FromStr>::from_str("4916438723201054042444019656898570362273600104737950912332066133041156126167531037694107148261784973791373741416916").unwrap(),
        <Fr as std::str::FromStr>::from_str("2051012425631842496541988522355880419451294963585346803386094216516304700350122283395835617980254056554805281571361").unwrap(),
        <Fr as std::str::FromStr>::from_str("3798823489123936531659900837301256429870899160904365915046540297606766455429074345739557832825816384178402417577821").unwrap(),
        <Fr as std::str::FromStr>::from_str("3488579879963562604710030332050196080084694331754868586303651819049352075134403758806818854394405488571472180191938").unwrap(),
        <Fr as std::str::FromStr>::from_str("4492130795397392969855395164821018678727495757238128952924370214482282522381731201562179077728641507166036172093705").unwrap(),
        <Fr as std::str::FromStr>::from_str("3317005458307697300506824179900015439367289946019131801104385804928666825382998172369005212575436969903579893443447").unwrap(),
    ));
    points.push
    ((
        <Fr as std::str::FromStr>::from_str("107731235112341014056601334649087826174537370769102664035726912801386121391377851228232171846167086556123468987581").unwrap(),
        <Fr as std::str::FromStr>::from_str("1963962790595933730523174120044002265904658564588760373139608454726106709892708541735292461364495865625076343970834").unwrap(),
        <Fr as std::str::FromStr>::from_str("3772344704532092886341369246824801251481136974060204850537714655166359576252103570869664311322574776526783576771648").unwrap(),
        <Fr as std::str::FromStr>::from_str("3369417395837999027367642060154424196933460733323625212490000298947532714508580040543260359372818031671953511014123").unwrap(),
        <Fr as std::str::FromStr>::from_str("4175821498239090704227498873059231216626902485432216794466274428029831154765291942158615924553999615542684931548444").unwrap(),
        <Fr as std::str::FromStr>::from_str("4596802191575459284564869970438643099629314686704701685272361610637041796430580294413835914610048876437144774613753").unwrap(),
    ));
    points.push
    ((
        <Fr as std::str::FromStr>::from_str("4526664922998713801045478841197727298417603686071738046788629310369593574178130371831574061517358789667110626054105").unwrap(),
        <Fr as std::str::FromStr>::from_str("2667786067761686000659307984720182570926199171791236728433548659000759742622466294074927440402001674004966896777550").unwrap(),
        <Fr as std::str::FromStr>::from_str("5129493253807975998519351351487075138002392217523224603409608224620269209607655478711987467602796326444862180226873").unwrap(),
        <Fr as std::str::FromStr>::from_str("4724524533410731353480555770462483048132518261498612055891908722191781632466157559343232579571932808201133332870995").unwrap(),
        <Fr as std::str::FromStr>::from_str("1399615561924155397199900618983918195829276511158286234509594550979958914007262146886462077109621809937162477157257").unwrap(),
        <Fr as std::str::FromStr>::from_str("2333105531115337450990014598715444884032287967420191090657707001012614193636424076105460534171237724334736546710446").unwrap(),
    ));
    points.push
    ((
        <Fr as std::str::FromStr>::from_str("4123527344569577669056282593988661411196763903614399606087980628084902313779076470707110640219624026814533876564722").unwrap(),
        <Fr as std::str::FromStr>::from_str("4826645822222627154915673570829569698396877361785810097819887583105702919324340126296380184169339911799185770577323").unwrap(),
        <Fr as std::str::FromStr>::from_str("3027353026192835011515604555215610362579318356058808091941560670325717683229132386678654797899058293060394241339067").unwrap(),
        <Fr as std::str::FromStr>::from_str("1893342279078375893720965821698231537818292912881407850560073443595346713982419102655915637930765470196489924638426").unwrap(),
        <Fr as std::str::FromStr>::from_str("2987066520006040393510041781578790694448032988620567180549808503907388510730439170598442037636574758551237326517585").unwrap(),
        <Fr as std::str::FromStr>::from_str("5359779630837471919145405238596268591478195733546546739332100097411048452487104831506370355931579775006693301412204").unwrap(),
    ));
    points.push
    ((
        <Fr as std::str::FromStr>::from_str("4912629693851117772591702754589926177602243346341736359735620883075700657248663514800994171572416686862701598040294").unwrap(),
        <Fr as std::str::FromStr>::from_str("2794185062119289427523238682792616130309230555534887179162536305702134719463420582069235713945987038549058324304842").unwrap(),
        <Fr as std::str::FromStr>::from_str("3668223185428705024105634945468964677340747480621538612583082295495362070898686851667449577863086303167734794958118").unwrap(),
        <Fr as std::str::FromStr>::from_str("1885533985152336743493791299787961985646628264863373253608270911882442035474983148909516194256200071297408931047513").unwrap(),
        <Fr as std::str::FromStr>::from_str("96577215787938354987539681438019148827270900406281053757281455870574490941975371463616142503892007018305065354990").unwrap(),
        <Fr as std::str::FromStr>::from_str("4590975612751681948840858609050355920090572361116944721088634123872268678971064004628732396556002735369572335641001").unwrap(),
    ));
    points.push
    ((
        <Fr as std::str::FromStr>::from_str("1911057726536241295027707664426124497399379763866398366535692190141197755421585992526481373383742936016531519006971").unwrap(),
        <Fr as std::str::FromStr>::from_str("4767708062186886007204177389565948024439321902322222101514656319135279446572606976792211881563818963207465590202391").unwrap(),
        <Fr as std::str::FromStr>::from_str("1907033740076880931314857394526569925369503087727191592341538222127746347304051994688995974775713845723667345181865").unwrap(),
        <Fr as std::str::FromStr>::from_str("1576971660752356241883555524353080145175730420061533971632037189630762455211281262541408736807970856276220818929667").unwrap(),
        <Fr as std::str::FromStr>::from_str("829503277983351805259157580650425085639218298706140884831455228147071806891928077167672811837789611280449655050214").unwrap(),
        <Fr as std::str::FromStr>::from_str("1756398464986740625913060543533736393413666564249415436116821095310039259507115581393336410392807276157514835984499").unwrap(),
    ));
    points.push
    ((
        <Fr as std::str::FromStr>::from_str("5024337537369005450568077688628459393381053291298376139972369846636479216251457004526489591434290880052740831323389").unwrap(),
        <Fr as std::str::FromStr>::from_str("243329854482099088875583668867255119261847081329212788111638378936806454156107058103196419674662040624666327192506").unwrap(),
        <Fr as std::str::FromStr>::from_str("4330163545923343810833214709269503909832448706659644591106183495009742425384776692209008020460802074034919882667156").unwrap(),
        <Fr as std::str::FromStr>::from_str("4746252481910923699031058431580647024618534370378744699707723637711718386819960443169105215582630805164566477915061").unwrap(),
        <Fr as std::str::FromStr>::from_str("4881904098552530317258681428870637086020848720937983433810060733832775275290507396271847059064285333750025015634555").unwrap(),
        <Fr as std::str::FromStr>::from_str("5533410041726567478516247267400321578296447732553340181372821608372766127968688407858150948235568435315038109321862").unwrap(),
    ));
    points.push
    ((
        <Fr as std::str::FromStr>::from_str("970488399982572621523416338345697693754377405072362363350911261719989500807800736022320682081707538986984963195903").unwrap(),
        <Fr as std::str::FromStr>::from_str("2889776142353439084565779169764141093305210999753671948339956878173834062323396587087003698941509191636828412358371").unwrap(),
        <Fr as std::str::FromStr>::from_str("3795770522092825694576578189765811809029572199748020189153305452621968802363915137823591748554135968046298920834815").unwrap(),
        <Fr as std::str::FromStr>::from_str("1370588897308522589002735579591748157760974937243710348850465791283211725475054776537830886721435077995422031781461").unwrap(),
        <Fr as std::str::FromStr>::from_str("1556482929005394304300371586952479480345522015024596090660772863731774844815426547463537002235381701522069766536218").unwrap(),
        <Fr as std::str::FromStr>::from_str("2440771835720874093456981432602912746214896524570412137643602573594284517320646831335034391207075189412012475745043").unwrap(),
    ));
    points.push
    ((
        <Fr as std::str::FromStr>::from_str("3499956327053992311789324315745279077218711522574396611145654815527085555633655891265950097145267897724655566156082").unwrap(),
        <Fr as std::str::FromStr>::from_str("4250726341623352245859193814958075653932439210552578930150640874506143643848176011936425569339283499036976370918547").unwrap(),
        <Fr as std::str::FromStr>::from_str("2277075619467092361075441637545474462708156505551901231294431119215104787945869965412279606012444168553048751531305").unwrap(),
        <Fr as std::str::FromStr>::from_str("5156632548154372308314396817752082262448465615602291512087163669501834087315996859928625857489259758171314588058684").unwrap(),
        <Fr as std::str::FromStr>::from_str("3549623035990464287836624902127543074314683544616644069999418936977157601068501815160870430922313809765697470461011").unwrap(),
        <Fr as std::str::FromStr>::from_str("4149192748600852083475900035990630534222222056341700086574476023821578193169627468582105359207744587896137324600702").unwrap(),
    ));

    println!("{}", "Prover 1000 zk-proofs computation".green());
    let group_map = <Affine as CommitmentCurve>::Map::setup();
    let mut start = Instant::now();

    let verifier_index = index.verifier_index();

    let tests = 0..1000;
    let mut batch = Vec::new();
    for test in tests.clone()
    {
        let (x1, y1, x2, y2, x3, y3) = points[test % 10];
        let s = (y2 - &y1) / &(x2 - &x1);

        let mut witness = vec![Fr::zero(); 8];
        witness[0] = Fr::one();
        witness[1] = x1;
        witness[2] = x2;
        witness[3] = x3;
        witness[4] = y1;
        witness[5] = y2;
        witness[6] = y3;
        witness[7] = s;

        // verify the circuit satisfiability by the computed witness
        assert_eq!(index.verify(&witness), true);

        let prev = {
            let k = ceil_log2(index.srs.get_ref().g.len());
            let chals : Vec<_> = (0..k).map(|_| Fr::rand(rng)).collect();
            let comm = {
                let b = DensePolynomial::from_coefficients_vec(b_poly_coefficients(&chals));
                index.srs.get_ref().commit_non_hiding(&b, None)
            };
            ( chals, comm )
        };

        // add the proof to the batch
        batch.push((&verifier_index,
            ProverProof::create::<DefaultFqSponge<Bn_382GParameters, SC>, DefaultFrSponge<Fr, SC>>(
                &group_map, &witness, &index, vec![prev], rng).unwrap()));

        print!("{:?}\r", test);
        io::stdout().flush().unwrap();
    }
    println!("{}{:?}", "Execution time: ".yellow(), start.elapsed());

    // verify one proof serially
    match ProverProof::verify::<DefaultFqSponge<Bn_382GParameters, SC>, DefaultFrSponge<Fr, SC>>(&group_map, &vec![batch[0].clone()], rng)
    {
        false => {panic!("Failure verifying the prover's proof")},
        true => {}
    }

    // verify the proofs in batch
    println!("{}", "Verifier zk-proofs verification".green());
    start = Instant::now();
    match ProverProof::verify::<DefaultFqSponge<Bn_382GParameters, SC>, DefaultFrSponge<Fr, SC>>(&group_map, &batch, rng)
    {
        false => {panic!("Failure verifying the prover's proofs in batch")},
        true => {println!("{}{:?}", "Execution time: ".yellow(), start.elapsed());}
    }
}

fn negative(index: &Index<Affine>)
where <Fr as std::str::FromStr>::Err : std::fmt::Debug
{
    // build non-satisfying witness
    let x1 = <Fr as std::str::FromStr>::from_str("5745565249002583850854246415085554015514779048799444283584466894422344259771098642674937363180389190528028396479257").unwrap();
    let y1 = <Fr as std::str::FromStr>::from_str("4813686258153460224790424816446786580277352317903647765375466966518317466807153459555932297360845688687684150591494").unwrap();
    let x2 = <Fr as std::str::FromStr>::from_str("3396151626077624174919604715379221669364304783678371552711527452445283433128730232001527953280138549542777514541384").unwrap();
    let y2 = <Fr as std::str::FromStr>::from_str("63554143638161614489708248187577450268846694052002135154886815922765033914172562010527524862363240669428855379409").unwrap();
    let x3 = <Fr as std::str::FromStr>::from_str("3777882049638999464453930301439616244826432091879659775472662153218285664108969981658902115718725487883901352544808").unwrap();
    let y3 = <Fr as std::str::FromStr>::from_str("547170016748527041421767087930546910879995562217027082548816642186645397743855029110891827292852193105112126622067").unwrap();

    let s = (y2 - &y1) / &(x2 - &x1);
    
    let mut witness = vec![Fr::zero(); 8];
    witness[0] = Fr::one();
    witness[1] = x1;
    witness[2] = x2;
    witness[3] = x3;
    witness[4] = y1;
    witness[5] = y2;
    witness[6] = y3;
    witness[7] = s;

    // verify the circuit negative satisfiability by the computed witness
    assert_eq!(index.verify(&witness), false);

    let group_map = <Affine as CommitmentCurve>::Map::setup();

    let rng = &mut OsRng;
    // create proof
    match ProverProof::create::<DefaultFqSponge<Bn_382GParameters, SC>, DefaultFrSponge<Fr, SC>>(&group_map, &witness, &index, vec![], rng)
    {
        Ok(_) => {panic!("Failure invalidating the witness")}
        _ => {}
    }
}
