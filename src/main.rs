use std::io::{BufRead, stdin};
use chumsky::prelude::*;

fn main() {
    for line in stdin().lock().lines() {
        let line = line.unwrap();
        let res  = parser().parse(line.as_str()).unwrap();
        let res  = res.iter()
            .map(|(e, c)| e.molar_mass() * *c as f64)
            .reduce(std::ops::Add::add)
            .unwrap();

        println!("{}", res);
    }
}

fn parser() -> impl Parser<char, Compound, Error = Simple<char>> {
    use Element::*;

    let lithium_1 = || choice((
        just("H").to(H),
        just("K").to(K),
    ));

    let lithium_2 = || choice((
        just("Li").to(Li),
        just("Na").to(Na),
        just("Rb").to(Rb),
        just("Cs").to(Cs),
        just("Fr").to(Fr),
    ));

    let beryllium_2 = || choice((
        just("Be").to(Be),
        just("Mg").to(Mg),
        just("Ca").to(Ca),
        just("Sr").to(Sr),
        just("Ba").to(Ba),
        just("Ra").to(Ra),
    ));

    let scandium_1 = || choice((
        just("Y").to(Y),
    ));

    let scandium_2 = || choice((
        just("Sc").to(Sc),
        just("Lu").to(Lu),
        just("Lr").to(Lr),
    ));

    let titanium_2 = || choice((
        just("Ti").to(Ti),
        just("Zr").to(Zr),
        just("Hf").to(Hf),
        just("Rf").to(Rf),
    ));

    let vanadium_1 = || choice((
        just("V").to(V),
    ));

    let vanadium_2 = || choice((
        just("Nb").to(Nb),
        just("Ta").to(Ta),
        just("Db").to(Db),
    ));

    let chromium_1 = || choice((
        just("W").to(W),
    ));

    let chromium_2 = || choice((
        just("Cr").to(Cr),
        just("Mo").to(Mo),
        just("Sg").to(Sg),
    ));

    let manganese_2 = || choice((
        just("Mn").to(Mn),
        just("Tc").to(Tc),
        just("Re").to(Re),
        just("Bh").to(Bh),
    ));

    let iron_2 = || choice((
        just("Fe").to(Fe),
        just("Ru").to(Ru),
        just("Os").to(Os),
        just("Hs").to(Hs),
    ));

    let cobalt_2 = || choice((
        just("Co").to(Co),
        just("Rh").to(Rh),
        just("Ir").to(Ir),
        just("Mt").to(Mt),
    ));

    let nickel_2 = || choice((
        just("Ni").to(Ni),
        just("Pd").to(Pd),
        just("Pt").to(Pt),
        just("Ds").to(Ds),
    ));

    let copper_2 = || choice((
        just("Cu").to(Cu),
        just("Ag").to(Ag),
        just("Au").to(Au),
        just("Rg").to(Rg),
    ));

    let zinc_2 = || choice((
        just("Zn").to(Zn),
        just("Cd").to(Cd),
        just("Hg").to(Hg),
        just("Cn").to(Cn),
    ));

    let boron_1 = || choice((
        just("B").to(B),
    ));

    let boron_2 = || choice((
        just("Al").to(Al),
        just("Ga").to(Ga),
        just("In").to(In),
        just("Tl").to(Tl),
        just("Nh").to(Nh),
    ));

    let carbon_1 = || choice((
        just("C").to(C),
    ));

    let carbon_2 = || choice((
        just("Si").to(Si),
        just("Ge").to(Ge),
        just("Sn").to(Sn),
        just("Pb").to(Pb),
        just("Fl").to(Fl),
    ));

    let nitrogen_1 = || choice((
        just("N").to(N),
        just("P").to(P),
    ));

    let nitrogen_2 = || choice((
        just("As").to(As),
        just("Sb").to(Sb),
        just("Bi").to(Bi),
        just("Mc").to(Mc),
    ));

    let oxygen_1 = || choice((
        just("O").to(O),
        just("S").to(S),
    ));

    let oxygen_2 = || choice((
        just("Se").to(Se),
        just("Te").to(Te),
        just("Po").to(Po),
        just("Lv").to(Lv),
    ));

    let fluorine_1 = || choice((
        just("F").to(F),
        just("I").to(I),
    ));

    let fluorine_2 = || choice((
        just("Cl").to(Cl),
        just("Br").to(Br),
        just("At").to(At),
        just("Ts").to(Ts),
    ));

    let neon_2 = || choice((
        just("He").to(He),
        just("Ne").to(Ne),
        just("Ar").to(Ar),
        just("Kr").to(Kr),
        just("Xe").to(Xe),
        just("Rn").to(Rn),
        just("Og").to(Og),
    ));

    let lanthanum_2 = || choice((
        just("La").to(La),
        just("Ce").to(Ce),
        just("Pr").to(Pr),
        just("Nd").to(Nd),
        just("Pm").to(Pm),
        just("Sm").to(Sm),
        just("Eu").to(Eu),
        just("Gd").to(Gd),
        just("Tb").to(Tb),
        just("Dy").to(Dy),
        just("Ho").to(Ho),
        just("Er").to(Er),
        just("Tm").to(Tm),
        just("Yb").to(Yb),
    ));

    let actinium_1 = || choice((
        just("U").to(U),
    ));

    let actinium_2 = || choice((
        just("Ac").to(Ac),
        just("Th").to(Th),
        just("Pa").to(Pa),
        just("Np").to(Np),
        just("Pu").to(Pu),
        just("Am").to(Am),
        just("Cm").to(Cm),
        just("Bk").to(Bk),
        just("Cf").to(Cf),
        just("Es").to(Es),
        just("Fm").to(Fm),
        just("Md").to(Md),
        just("No").to(No),
        just("Lr").to(Lr),
    ));

    let single = || choice((
        lithium_1(),
        scandium_1(),
        vanadium_1(),
        chromium_1(),
        boron_1(),
        carbon_1(),
        nitrogen_1(),
        oxygen_1(),
        fluorine_1(),
        actinium_1(),
    ));

    let double = || choice((
        lithium_2(),
        beryllium_2(),
        scandium_2(),
        titanium_2(),
        vanadium_2(),
        chromium_2(),
        manganese_2(),
        iron_2(),
        cobalt_2(),
        nickel_2(),
        copper_2(),
        zinc_2(),
        boron_2(),
        carbon_2(),
        nitrogen_2(),
        oxygen_2(),
        fluorine_2(),
        neon_2(),
        lanthanum_2(),
        actinium_2(),
    ));

    let element = || choice((
        double(),
        single(),
    ));

    let element_count = || element()
        .then(text::int(10).from_str().unwrapped())
        .or(element().map(|e| (e, 1)));

    element_count()
        .repeated()
        .at_least(1)
        .padded_by(text::whitespace())
}

type Compound = Vec<(Element, u16)>;

#[derive(Debug, Clone)]
enum Element {
    H, He,
    Li, Be, B, C, N, O, F, Ne,
    Na, Mg, Al, Si, P, S, Cl, Ar,
    K, Ca, Sc, Ti, V, Cr, Mn, Fe, Co, Ni, Cu, Zn, Ga, Ge, As, Se, Br, Kr,
    Rb, Sr, Y, Zr, Nb, Mo, Tc, Ru, Rh, Pd, Ag, Cd, In, Sn, Sb, Te, I, Xe,
    Cs, Ba, Hf, Ta, W, Re, Os, Ir, Pt, Au, Hg, Tl, Pb, Bi, Po, At, Rn,
    La, Ce, Pr, Nd, Pm, Sm, Eu, Gd, Tb, Dy, Ho, Er, Tm, Yb, Lu,
    Fr, Ra, Rf, Db, Sg, Bh, Hs, Mt, Ds, Rg, Cn, Nh, Fl, Mc, Lv, Ts, Og,
    Ac, Th, Pa, U, Np, Pu, Am, Cm, Bk, Cf, Es, Fm, Md, No, Lr,
}

impl Element {
    fn molar_mass(&self) -> f64 {
        use Element::*;

        match self {
            H  => 1.00794,
            He => 4.002602,
            Li => 6.941,
            Be => 9.0121831,
            B  => 10.811,
            C  => 12.011,
            N  => 14.00674,
            O  => 15.9994,
            F  => 18.998403163,
            Na => 22.989768,
            Mg => 24.3050,
            Al => 26.9815385,
            _ => todo!(),
        }
    }
}

