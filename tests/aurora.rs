use polyanya::Mesh;

const IGNORE_KNOWN_FAILURE: bool = true;

macro_rules! assert_delta {
    ($x:expr, $y:expr) => {
        let val = $x;
        if !((val - $y).abs() < 0.001) {
            assert_eq!(val, $y);
        }
    };
}

macro_rules! known_failure {
    ($x:expr, $y:expr) => {
        let val = $x;
        if !IGNORE_KNOWN_FAILURE {
            if !((val - $y).abs() < 0.001) {
                assert_eq!(val, $y);
            }
        } else {
            if !((val - $y).abs() > 0.001) {
                assert_eq!(val, $y);
            }
        }
    };
}

fn aurora_mesh() -> Mesh {
    Mesh::from_file("meshes/aurora-merged.mesh".into())
}

#[test]
fn aurora_tqoopurmvw() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([749.0, 97.0], [749.0, 104.0]).len, 7.0);
}
#[test]
fn aurora_cclvlrnsaq() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([8.0, 646.0], [11.0, 642.0]).len, 5.0);
}
#[test]
fn aurora_lokgejgyeu() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([271.0, 287.0], [276.0, 283.0]).len, 6.40312);
}
#[test]
fn aurora_fffwgyojmu() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([814.0, 570.0], [810.0, 571.0]).len, 4.12311);
}
#[test]
fn aurora_znopuuwfbo() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([579.0, 169.0], [575.0, 163.0]).len, 7.2111);
}
#[test]
fn aurora_xgykiahuvy() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([548.0, 325.0], [551.0, 321.0]).len, 5.0);
}
#[test]
fn aurora_xqnwiicacp() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([89.0, 124.0], [95.0, 127.0]).len, 6.7082);
}
#[test]
fn aurora_lrocxpovdd() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([195.0, 114.0], [192.0, 120.0]).len, 6.7082);
}
#[test]
fn aurora_zkupujzlnz() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([146.0, 290.0], [142.0, 287.0]).len, 5.0);
}
#[test]
fn aurora_yxwciqahzj() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([114.0, 437.0], [117.0, 433.0]).len, 5.0);
}
#[test]
fn aurora_pgnwizkzsx() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([358.0, 666.0], [368.0, 669.0]).len, 10.4403);
}
#[test]
fn aurora_xetnfuiwkg() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([840.0, 252.0], [847.0, 248.0]).len, 8.06226);
}
#[test]
fn aurora_mccasdpiih() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([428.0, 300.0], [418.0, 301.0]).len, 10.0499);
}
#[test]
fn aurora_pipwqwqkcb() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([895.0, 646.0], [903.0, 641.0]).len, 9.53663);
}
#[test]
fn aurora_rimxktqqvo() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([940.0, 471.0], [929.0, 473.0]).len, 11.1803);
}
#[test]
fn aurora_xxajehdmel() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([612.0, 654.0], [607.0, 645.0]).len, 10.2956);
}
#[test]
fn aurora_lfqktfrhks() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([610.0, 451.0], [617.0, 454.0]).len, 7.61577);
}
#[test]
fn aurora_ubisazfsnw() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([300.0, 358.0], [291.0, 351.0]).len, 11.4018);
}
#[test]
fn aurora_naonzhadop() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([10.0, 655.0], [18.0, 663.0]).len, 11.3137);
}
#[test]
fn aurora_uatyayxqgc() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([490.0, 682.0], [500.0, 680.0]).len, 10.198);
}
#[test]
fn aurora_azczlysdps() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([70.0, 554.0], [73.0, 540.0]).len, 14.3178);
}
#[test]
fn aurora_ztmtlengrf() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([717.0, 172.0], [729.0, 170.0]).len, 12.1655);
}
#[test]
fn aurora_kwjvbxlbsv() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([521.0, 706.0], [508.0, 702.0]).len, 13.6015);
}
#[test]
fn aurora_rkdjjhlwzf() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([717.0, 683.0], [731.0, 681.0]).len, 14.1421);
}
#[test]
fn aurora_tqvljiiugf() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([392.0, 601.0], [405.0, 606.0]).len, 13.9284);
}
#[test]
fn aurora_bnvsxwbniz() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([347.0, 272.0], [348.0, 285.0]).len, 13.0384);
}
#[test]
fn aurora_gmqzfevnpo() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([229.0, 639.0], [240.0, 631.0]).len, 13.6015);
}
#[test]
fn aurora_vjzvhpyyui() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([749.0, 423.0], [739.0, 412.0]).len, 14.8661);
}
#[test]
fn aurora_hozkxmukgo() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([485.0, 297.0], [472.0, 300.0]).len, 13.3417);
}
#[test]
fn aurora_hipbpqjqbq() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([628.0, 302.0], [638.0, 309.0]).len, 12.2066);
}
#[test]
fn aurora_mcxawlojjj() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([386.0, 468.0], [374.0, 483.0]).len, 19.2094);
}
#[test]
fn aurora_niujsezumi() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([483.0, 499.0], [468.0, 487.0]).len, 19.2094);
}
#[test]
fn aurora_ltcttxevlg() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([869.0, 424.0], [854.0, 416.0]).len, 17.0);
}
#[test]
fn aurora_lcvljzpkvd() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([514.0, 346.0], [499.0, 337.0]).len, 17.4929);
}
#[test]
fn aurora_nwuwiqvrek() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([208.0, 251.0], [218.0, 238.0]).len, 16.4012);
}
#[test]
fn aurora_dwckffntix() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([676.0, 100.0], [669.0, 115.0]).len, 16.5529);
}
#[test]
fn aurora_zfgogbdsvj() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([881.0, 382.0], [897.0, 375.0]).len, 17.4642);
}
#[test]
fn aurora_ejuthidzms() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([927.0, 418.0], [932.0, 404.0]).len, 14.8661);
}
#[test]
fn aurora_pxqlgzyqlh() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([323.0, 99.0], [309.0, 90.0]).len, 16.6433);
}
#[test]
fn aurora_piamvgpjcj() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([354.0, 118.0], [337.0, 112.0]).len, 18.0278);
}
#[test]
fn aurora_oecnbcxell() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([719.0, 111.0], [700.0, 114.0]).len, 19.2354);
}
#[test]
fn aurora_jxrzthavjn() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([863.0, 142.0], [884.0, 147.0]).len, 21.587);
}
#[test]
fn aurora_qwbdickslr() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([442.0, 456.0], [447.0, 477.0]).len, 21.587);
}
#[test]
fn aurora_fqqfzpzxwt() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([903.0, 419.0], [911.0, 436.0]).len, 18.7883);
}
#[test]
fn aurora_fimkyomjei() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([813.0, 242.0], [790.0, 244.0]).len, 23.0868);
}
#[test]
fn aurora_iajrfxitdi() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([604.0, 197.0], [619.0, 214.0]).len, 22.6716);
}
#[test]
fn aurora_gicnryiync() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([274.0, 273.0], [258.0, 257.0]).len, 22.6274);
}
#[test]
fn aurora_uwkqffvmfl() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([313.0, 547.0], [317.0, 526.0]).len, 21.3776);
}
#[test]
fn aurora_jgujqoizee() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([134.0, 674.0], [116.0, 665.0]).len, 20.1246);
}
#[test]
fn aurora_hrbvuysztl() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([464.0, 326.0], [474.0, 343.0]).len, 19.7231);
}
#[test]
fn aurora_bduqitjszq() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([58.0, 404.0], [45.0, 385.0]).len, 23.0217);
}
#[test]
fn aurora_jwoekyskqr() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([55.0, 166.0], [76.0, 182.0]).len, 26.4008);
}
#[test]
fn aurora_nfmthareeh() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([14.0, 387.0], [40.0, 390.0]).len, 26.1725);
}
#[test]
fn aurora_icvutckzev() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([493.0, 444.0], [470.0, 436.0]).len, 24.3516);
}
#[test]
fn aurora_pjxcvzmwml() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([653.0, 394.0], [674.0, 407.0]).len, 24.6982);
}
#[test]
fn aurora_afgyrtixjj() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([836.0, 122.0], [857.0, 111.0]).len, 23.7065);
}
#[test]
fn aurora_uhgmlxvtov() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([27.0, 591.0], [42.0, 573.0]).len, 23.455);
}
#[test]
fn aurora_uoztiyeojg() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([761.0, 368.0], [740.0, 358.0]).len, 23.2594);
}
#[test]
fn aurora_dbhbzmhfzn() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([693.0, 312.0], [682.0, 298.0]).len, 22.1812);
}
#[test]
fn aurora_uyfyhnabgx() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([126.0, 183.0], [103.0, 189.0]).len, 23.7697);
}
#[test]
fn aurora_ofylakmlfx() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([545.0, 503.0], [517.0, 507.0]).len, 28.2843);
}
#[test]
fn aurora_iqfbrbxfdl() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([812.0, 550.0], [809.0, 577.0]).len, 27.1662);
}
#[test]
fn aurora_tfoikavpmq() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([885.0, 286.0], [860.0, 299.0]).len, 28.178);
}
#[test]
fn aurora_koqmgituss() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([818.0, 292.0], [795.0, 306.0]).len, 26.9258);
}
#[test]
fn aurora_qxjutyfmxo() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([467.0, 205.0], [446.0, 185.0]).len, 29.0);
}
#[test]
fn aurora_swizcmghvv() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([114.0, 464.0], [143.0, 469.0]).len, 29.4279);
}
#[test]
fn aurora_ltozfxpqfs() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([717.0, 368.0], [741.0, 381.0]).len, 27.2947);
}
#[test]
fn aurora_zcxrpufpyt() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([432.0, 259.0], [462.0, 258.0]).len, 30.0167);
}
#[test]
fn aurora_udzfvsnaek() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([134.0, 434.0], [136.0, 405.0]).len, 29.0689);
}
#[test]
fn aurora_glelhvhyoy() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([305.0, 683.0], [275.0, 681.0]).len, 30.0666);
}
#[test]
fn aurora_kkrhngdxlj() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([336.0, 321.0], [336.0, 353.0]).len, 32.0);
}
#[test]
fn aurora_zqxegsjjvo() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([646.0, 410.0], [642.0, 444.0]).len, 34.2345);
}
#[test]
fn aurora_oxahqsdlhg() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([255.0, 382.0], [281.0, 362.0]).len, 32.8024);
}
#[test]
fn aurora_nsueqjyjkd() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([326.0, 509.0], [298.0, 520.0]).len, 30.313);
}
#[test]
fn aurora_rxweizlrml() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([980.0, 447.0], [965.0, 418.0]).len, 32.6497);
}
#[test]
fn aurora_kmpgztxeru() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([645.0, 591.0], [619.0, 573.0]).len, 31.6969);
}
#[test]
fn aurora_zxqlhvbmyi() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([123.0, 465.0], [142.0, 492.0]).len, 33.0151);
}
#[test]
fn aurora_fdsvxkfbdr() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([387.0, 227.0], [358.0, 239.0]).len, 31.3847);
}
#[test]
fn aurora_ollcjefcog() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([383.0, 340.0], [390.0, 370.0]).len, 32.8629);
}
#[test]
fn aurora_qbfmapettn() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([100.0, 290.0], [129.0, 303.0]).len, 33.7275);
}
#[test]
fn aurora_afknbgneln() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([555.0, 646.0], [528.0, 624.0]).len, 34.8281);
}
#[test]
fn aurora_zpzggakaey() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([457.0, 673.0], [493.0, 670.0]).len, 36.1248);
}
#[test]
fn aurora_nmjicxusdc() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([13.0, 622.0], [28.0, 604.0]).len, 37.4046);
}
#[test]
fn aurora_ryhorwlrwq() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([278.0, 92.0], [314.0, 98.0]).len, 36.4966);
}
#[test]
fn aurora_dvwuixprla() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([527.0, 541.0], [520.0, 506.0]).len, 35.6931);
}
#[test]
fn aurora_jgfvztsrfg() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([774.0, 216.0], [805.0, 216.0]).len, 35.4222);
}
#[test]
fn aurora_ukoufeakrd() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([229.0, 698.0], [199.0, 714.0]).len, 34.0);
}
#[test]
fn aurora_nkuqpkddgh() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([592.0, 177.0], [607.0, 146.0]).len, 34.4384);
}
#[test]
fn aurora_snqcuuaxqu() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([889.0, 512.0], [874.0, 482.0]).len, 33.541);
}
#[test]
fn aurora_bwvltefrjj() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([251.0, 98.0], [222.0, 115.0]).len, 33.6155);
}
#[test]
fn aurora_rkcwoueoil() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([903.0, 380.0], [935.0, 352.0]).len, 42.5206);
}
#[test]
fn aurora_zhldwdmxyx() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([537.0, 193.0], [567.0, 219.0]).len, 39.6989);
}
#[test]
fn aurora_ayqpqyarhu() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([230.0, 281.0], [263.0, 261.0]).len, 39.0);
}
#[test]
fn aurora_fmuurqbxdl() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([636.0, 85.0], [603.0, 106.0]).len, 39.5876);
}
#[test]
fn aurora_hjylrcsyxd() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([620.0, 235.0], [641.0, 268.0]).len, 39.2753);
}
#[test]
fn aurora_qbnkzqyvdf() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([167.0, 235.0], [201.0, 255.0]).len, 39.4462);
}
#[test]
fn aurora_dgkripqwmi() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([315.0, 338.0], [337.0, 307.0]).len, 38.0132);
}
#[test]
fn aurora_mcfucosfai() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([72.0, 402.0], [101.0, 431.0]).len, 41.0122);
}
#[test]
fn aurora_kbmyimwdvp() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([632.0, 742.0], [623.0, 704.0]).len, 39.108);
}
#[test]
fn aurora_awqvvjuhho() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([709.0, 584.0], [741.0, 599.0]).len, 39.4709);
}
#[test]
fn aurora_nkfmnziula() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([908.0, 292.0], [911.0, 257.0]).len, 42.4083);
}
#[test]
fn aurora_foobsuxdyp() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([104.0, 564.0], [145.0, 566.0]).len, 42.1045);
}
#[test]
fn aurora_xdhnlfegrw() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([272.0, 92.0], [233.0, 106.0]).len, 41.4367);
}
#[test]
fn aurora_bhkneqnlpl() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([322.0, 296.0], [283.0, 277.0]).len, 43.382);
}
#[test]
fn aurora_gtbzkyhimv() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([442.0, 460.0], [398.0, 467.0]).len, 44.5533);
}
#[test]
fn aurora_syetcbymyd() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([774.0, 350.0], [731.0, 362.0]).len, 44.643);
}
#[test]
fn aurora_pbqyinexws() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([118.0, 721.0], [95.0, 686.0]).len, 41.8808);
}
#[test]
fn aurora_wobhgwixsf() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([569.0, 163.0], [607.0, 183.0]).len, 42.9418);
}
#[test]
fn aurora_ubtxfugbii() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([87.0, 632.0], [122.0, 610.0]).len, 41.3401);
}
#[test]
fn aurora_ohhdtykvjd() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([214.0, 529.0], [176.0, 551.0]).len, 43.909);
}
#[test]
fn aurora_tiahhmrcfv() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([75.0, 148.0], [113.0, 141.0]).len, 46.5038);
}
#[test]
fn aurora_zwqwhnezqp() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([572.0, 486.0], [533.0, 464.0]).len, 44.7772);
}
#[test]
fn aurora_ugcidugjnv() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([456.0, 589.0], [492.0, 618.0]).len, 46.2277);
}
#[test]
fn aurora_zyuxfabxaa() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([488.0, 493.0], [454.0, 456.0]).len, 50.2769);
}
#[test]
fn aurora_mxjteqgzvd() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([835.0, 549.0], [788.0, 555.0]).len, 47.3814);
}
#[test]
fn aurora_putguhurkp() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([191.0, 701.0], [237.0, 714.0]).len, 47.8017);
}
#[test]
fn aurora_zkzdnpfeeu() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([909.0, 261.0], [897.0, 279.0]).len, 47.9312);
}
#[test]
fn aurora_jbwqdimbnl() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([614.0, 608.0], [650.0, 572.0]).len, 50.9117);
}
#[test]
fn aurora_obbjjngndf() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([487.0, 539.0], [452.0, 574.0]).len, 49.4975);
}
#[test]
fn aurora_iylyasrbkh() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([806.0, 370.0], [765.0, 348.0]).len, 46.5296);
}
#[test]
fn aurora_qyaslupbtz() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([905.0, 592.0], [859.0, 570.0]).len, 50.9902);
}
#[test]
fn aurora_snuwroswod() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([713.0, 657.0], [711.0, 604.0]).len, 53.0377);
}
#[test]
fn aurora_myjjtyrrby() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([468.0, 288.0], [428.0, 324.0]).len, 53.8145);
}
#[test]
fn aurora_srykvkkzfn() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([567.0, 158.0], [544.0, 140.0]).len, 51.2681);
}
#[test]
fn aurora_dyklqmsobv() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([488.0, 138.0], [467.0, 156.0]).len, 47.5703);
}
#[test]
fn aurora_kmszholbgv() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([631.0, 402.0], [620.0, 353.0]).len, 50.3406);
}
#[test]
fn aurora_rvqwtjtdwy() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([681.0, 216.0], [707.0, 178.0]).len, 52.9495);
}
#[test]
fn aurora_mvfkawqhtf() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([801.0, 543.0], [851.0, 551.0]).len, 50.636);
}
#[test]
fn aurora_ecykohikwk() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([549.0, 600.0], [503.0, 621.0]).len, 50.5668);
}
#[test]
fn aurora_qucwiazxhd() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([976.0, 624.0], [923.0, 629.0]).len, 53.2353);
}
#[test]
fn aurora_axojabfseg() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([965.0, 415.0], [1023.0, 412.0]).len, 58.0775);
}
#[test]
fn aurora_qbqoabrauy() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([902.0, 343.0], [923.0, 295.0]).len, 52.3927);
}
#[test]
fn aurora_aqxmclxbxj() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([228.0, 212.0], [182.0, 239.0]).len, 53.3385);
}
#[test]
fn aurora_kpqijgckeh() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([795.0, 680.0], [747.0, 700.0]).len, 52.0);
}
#[test]
fn aurora_tikgglggxp() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([710.0, 398.0], [686.0, 434.0]).len, 56.8737);
}
#[test]
fn aurora_ldzpwrsyox() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([318.0, 166.0], [294.0, 209.0]).len, 55.5646);
}
#[test]
fn aurora_sseyjoioia() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([166.0, 303.0], [209.0, 269.0]).len, 54.8759);
}
#[test]
fn aurora_xybajfredd() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([117.0, 105.0], [70.0, 133.0]).len, 54.7083);
}
#[test]
fn aurora_nlssjzvuuw() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([544.0, 302.0], [593.0, 319.0]).len, 51.8652);
}
#[test]
fn aurora_ixoktyieri() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([658.0, 344.0], [704.0, 319.0]).len, 52.3546);
}
#[test]
fn aurora_csbopjapgl() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([426.0, 140.0], [406.0, 109.0]).len, 56.8809);
}
#[test]
fn aurora_dfisuwipfj() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([245.0, 594.0], [236.0, 631.0]).len, 58.3086);
}
#[test]
fn aurora_jgfvqzifnh() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([210.0, 530.0], [155.0, 525.0]).len, 56.8212);
}
#[test]
fn aurora_pefylzkkor() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([255.0, 572.0], [210.0, 531.0]).len, 60.8769);
}
#[test]
fn aurora_gsnkoqdouv() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([21.0, 585.0], [61.0, 618.0]).len, 57.9557);
}
#[test]
fn aurora_vwngwcfflp() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([114.0, 237.0], [117.0, 281.0]).len, 56.7178);
}
#[test]
fn aurora_zvormnttby() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([1021.0, 340.0], [967.0, 346.0]).len, 57.1555);
}
#[test]
fn aurora_loqxyyjxas() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([644.0, 340.0], [598.0, 375.0]).len, 57.8014);
}
#[test]
fn aurora_jfrxqngkhh() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([123.0, 157.0], [68.0, 178.0]).len, 58.8727);
}
#[test]
fn aurora_lkzmpnxark() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([536.0, 261.0], [562.0, 307.0]).len, 56.3745);
}
#[test]
fn aurora_ecedjsxeal() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([672.0, 72.0], [724.0, 79.0]).len, 60.6906);
}
#[test]
fn aurora_bdreokcnjx() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([508.0, 318.0], [494.0, 295.0]).len, 59.7026);
}
#[test]
fn aurora_qmfgzxgdky() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([676.0, 115.0], [644.0, 94.0]).len, 59.7014);
}
#[test]
fn aurora_pcvzczftjt() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([127.0, 281.0], [102.0, 337.0]).len, 63.9275);
}
#[test]
fn aurora_jhbuitkxmc() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([629.0, 627.0], [569.0, 644.0]).len, 65.0856);
}
#[test]
fn aurora_sekvygellt() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([472.0, 544.0], [531.0, 530.0]).len, 60.7235);
}
#[test]
fn aurora_xeoudhujux() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([223.0, 646.0], [249.0, 674.0]).len, 62.6869);
}
#[test]
fn aurora_lmaeathjkt() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([447.0, 458.0], [425.0, 415.0]).len, 61.8827);
}
#[test]
fn aurora_cwfmcgbwoy() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([147.0, 390.0], [86.0, 376.0]).len, 62.5859);
}
#[test]
fn aurora_mbgbjfyrqt() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([382.0, 542.0], [432.0, 506.0]).len, 61.6117);
}
#[test]
fn aurora_rjlaitsfro() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([470.0, 547.0], [541.0, 548.0]).len, 71.007);
}
#[test]
fn aurora_lixstjnlgp() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([771.0, 393.0], [832.0, 372.0]).len, 66.3305);
}
#[test]
fn aurora_rfexakfskk() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([226.0, 502.0], [194.0, 452.0]).len, 65.6878);
}
#[test]
fn aurora_ypjzgablsj() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([668.0, 535.0], [695.0, 595.0]).len, 69.0304);
}
#[test]
fn aurora_wxbuwoxros() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([577.0, 669.0], [523.0, 635.0]).len, 63.8122);
}
#[test]
fn aurora_qotlazdqht() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([454.0, 341.0], [494.0, 396.0]).len, 69.223);
}
#[test]
fn aurora_qjnrbuqdtq() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([6.0, 410.0], [56.0, 369.0]).len, 67.448);
}
#[test]
fn aurora_wamcqpvidd() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([499.0, 552.0], [525.0, 491.0]).len, 66.3099);
}
#[test]
fn aurora_svphshvlfj() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([334.0, 566.0], [390.0, 603.0]).len, 67.1193);
}
#[test]
fn aurora_qvruqqwtgy() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([720.0, 180.0], [731.0, 226.0]).len, 63.0995);
}
#[test]
fn aurora_ajlrijtrvg() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([201.0, 399.0], [265.0, 375.0]).len, 68.352);
}
#[test]
fn aurora_opfaeoekze() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([809.0, 595.0], [741.0, 611.0]).len, 69.857);
}
#[test]
fn aurora_ndanmwixbn() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([697.0, 576.0], [762.0, 559.0]).len, 67.1863);
}
#[test]
fn aurora_uwqzsazjwf() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([633.0, 584.0], [691.0, 544.0]).len, 70.4557);
}
#[test]
fn aurora_puuskzgrhp() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([58.0, 653.0], [123.0, 630.0]).len, 71.0863);
}
#[test]
fn aurora_asfmfthgfd() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([659.0, 165.0], [731.0, 161.0]).len, 72.111);
}
#[test]
fn aurora_hdpjdfwink() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([769.0, 647.0], [758.0, 666.0]).len, 70.1669);
}
#[test]
fn aurora_thuevkvoma() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([615.0, 651.0], [543.0, 643.0]).len, 72.4431);
}
#[test]
fn aurora_traryjokzh() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([215.0, 111.0], [191.0, 74.0]).len, 71.0889);
}
#[test]
fn aurora_rxtzzzramd() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([442.0, 123.0], [382.0, 131.0]).len, 69.5767);
}
#[test]
fn aurora_spdcautcea() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([129.0, 209.0], [177.0, 245.0]).len, 71.5466);
}
#[test]
fn aurora_yhvayfgsmk() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([174.0, 372.0], [184.0, 299.0]).len, 73.6817);
}
#[test]
fn aurora_cvitaaccvd() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([518.0, 354.0], [513.0, 281.0]).len, 75.4362);
}
#[test]
fn aurora_jvweurbmqd() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([664.0, 269.0], [729.0, 305.0]).len, 74.3034);
}
#[test]
fn aurora_zctckcchkd() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([965.0, 154.0], [905.0, 192.0]).len, 71.4347);
}
#[test]
fn aurora_nfwjckuhcj() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([307.0, 617.0], [283.0, 687.0]).len, 74.0999);
}
#[test]
fn aurora_kdpnscjbak() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([270.0, 200.0], [324.0, 250.0]).len, 75.4775);
}
#[test]
fn aurora_textnxdkro() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([652.0, 519.0], [629.0, 580.0]).len, 74.2269);
}
#[test]
fn aurora_fpnjkwrife() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([770.0, 119.0], [711.0, 76.0]).len, 73.1276);
}
#[test]
fn aurora_gzizqbqykq() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([201.0, 286.0], [163.0, 333.0]).len, 70.0244);
}
#[test]
fn aurora_olrbzhvigq() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([664.0, 633.0], [591.0, 607.0]).len, 77.4919);
}
#[test]
fn aurora_rozegpzwdc() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([706.0, 537.0], [706.0, 609.0]).len, 77.2939);
}
#[test]
fn aurora_svcchqvzie() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([494.0, 134.0], [528.0, 179.0]).len, 79.5731);
}
#[test]
fn aurora_uzmbiffjnr() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([453.0, 520.0], [436.0, 534.0]).len, 73.7515);
}
#[test]
fn aurora_hupliftcfd() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([182.0, 354.0], [216.0, 322.0]).len, 74.9783);
}
#[test]
fn aurora_pdhxabyjwa() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([543.0, 643.0], [498.0, 593.0]).len, 76.8788);
}
#[test]
fn aurora_ueqozknucf() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([278.0, 489.0], [336.0, 514.0]).len, 76.9914);
}
#[test]
fn aurora_dienzwnvmw() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([596.0, 151.0], [526.0, 164.0]).len, 74.2301);
}
#[test]
fn aurora_gotkxuqgfo() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([719.0, 606.0], [766.0, 543.0]).len, 81.5821);
}
#[test]
fn aurora_gfelcxtpsa() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([367.0, 719.0], [299.0, 752.0]).len, 75.5844);
}
#[test]
fn aurora_zpiulakeqw() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([706.0, 524.0], [778.0, 491.0]).len, 79.2023);
}
#[test]
fn aurora_tblzcvueuo() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([913.0, 482.0], [842.0, 453.0]).len, 84.5886);
}
#[test]
fn aurora_bwvytavkae() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([451.0, 146.0], [436.0, 80.0]).len, 81.8474);
}
#[test]
fn aurora_lbsmsabdei() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([252.0, 560.0], [322.0, 602.0]).len, 81.6333);
}
#[test]
fn aurora_cdhvyyjilx() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([930.0, 540.0], [911.0, 492.0]).len, 80.5801);
}
#[test]
fn aurora_ehjpxurrzi() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([545.0, 172.0], [620.0, 201.0]).len, 80.4114);
}
#[test]
fn aurora_alvscednqp() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([839.0, 525.0], [894.0, 569.0]).len, 81.5683);
}
#[test]
fn aurora_vrxagadgek() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([528.0, 461.0], [491.0, 514.0]).len, 82.9281);
}
#[test]
fn aurora_odqtvdkmag() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([678.0, 368.0], [648.0, 425.0]).len, 79.0596);
}
#[test]
fn aurora_xcdkybdpof() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([527.0, 325.0], [451.0, 350.0]).len, 80.0062);
}
#[test]
fn aurora_irdxiqmaoe() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([878.0, 245.0], [932.0, 302.0]).len, 86.1604);
}
#[test]
fn aurora_zjohkbfmxt() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([41.0, 509.0], [118.0, 525.0]).len, 82.2626);
}
#[test]
fn aurora_sbeyxeyqfo() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([798.0, 442.0], [869.0, 484.0]).len, 83.0759);
}
#[test]
fn aurora_dkemlbwupx() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([820.0, 400.0], [795.0, 334.0]).len, 83.4117);
}
#[test]
fn aurora_nkwaezczwi() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([94.0, 406.0], [143.0, 476.0]).len, 85.4459);
}
#[test]
fn aurora_iwuclbazex() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([102.0, 375.0], [24.0, 405.0]).len, 84.9267);
}
#[test]
fn aurora_sgysfbwwes() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([169.0, 179.0], [138.0, 195.0]).len, 86.0704);
}
#[test]
fn aurora_xpfanbyfkx() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([624.0, 290.0], [597.0, 233.0]).len, 84.8778);
}
#[test]
fn aurora_szmqdlrtwg() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([744.0, 158.0], [712.0, 80.0]).len, 87.0808);
}
#[test]
fn aurora_yblzivlrkj() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([526.0, 394.0], [521.0, 332.0]).len, 80.4314);
}
#[test]
fn aurora_jnclnqyqfm() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([764.0, 636.0], [845.0, 667.0]).len, 87.3999);
}
#[test]
fn aurora_xcyqanzgru() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([524.0, 587.0], [445.0, 625.0]).len, 88.3247);
}
#[test]
fn aurora_bvabwpiefd() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([592.0, 241.0], [518.0, 189.0]).len, 90.4434);
}
#[test]
fn aurora_bcshxylbql() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([527.0, 565.0], [606.0, 533.0]).len, 88.2341);
}
#[test]
fn aurora_cckzkuqeyc() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([514.0, 320.0], [555.0, 244.0]).len, 86.3539);
}
#[test]
fn aurora_kvkflwypda() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([320.0, 660.0], [256.0, 603.0]).len, 89.4251);
}
#[test]
fn aurora_dbxytqzlqw() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([266.0, 645.0], [267.0, 561.0]).len, 86.1849);
}
#[test]
fn aurora_ircjpvmrhu() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([708.0, 567.0], [796.0, 557.0]).len, 88.5664);
}
#[test]
fn aurora_djwzgckjrr() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([487.0, 167.0], [570.0, 136.0]).len, 88.6002);
}
#[test]
fn aurora_qqsqafolku() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([674.0, 378.0], [746.0, 372.0]).len, 84.3871);
}
#[test]
fn aurora_yvdamhvjee() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([609.0, 609.0], [657.0, 532.0]).len, 95.4461);
}
#[test]
fn aurora_dbnpgyavgi() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([13.0, 326.0], [28.0, 413.0]).len, 93.7099);
}
#[test]
fn aurora_hypohhzkfp() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([591.0, 609.0], [530.0, 678.0]).len, 96.2778);
}
#[test]
fn aurora_caousbnkmm() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([75.0, 144.0], [167.0, 155.0]).len, 93.1209);
}
#[test]
fn aurora_wnqakepmiu() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([244.0, 617.0], [218.0, 530.0]).len, 92.9498);
}
#[test]
fn aurora_unddajzepc() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([820.0, 495.0], [823.0, 587.0]).len, 94.1679);
}
#[test]
fn aurora_dqcuvnbpvl() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([321.0, 624.0], [228.0, 608.0]).len, 94.6241);
}
#[test]
fn aurora_psqoistner() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([444.0, 121.0], [392.0, 163.0]).len, 93.1583);
}
#[test]
fn aurora_nitmiglcik() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([524.0, 640.0], [469.0, 566.0]).len, 93.3364);
}
#[test]
fn aurora_yxwqhrnlal() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([684.0, 309.0], [701.0, 333.0]).len, 91.1695);
}
#[test]
fn aurora_ryhxwitcvg() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([909.0, 548.0], [969.0, 614.0]).len, 96.8366);
}
#[test]
fn aurora_bvxwwsxgzq() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([473.0, 245.0], [562.0, 253.0]).len, 95.3451);
}
#[test]
fn aurora_eypbzpzyld() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([543.0, 309.0], [498.0, 231.0]).len, 97.0411);
}
#[test]
fn aurora_zrqzretgvt() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([889.0, 615.0], [808.0, 567.0]).len, 94.2606);
}
#[test]
fn aurora_byieluofhe() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([722.0, 228.0], [729.0, 144.0]).len, 92.2824);
}
#[test]
fn aurora_pnpfrwdlud() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([165.0, 263.0], [249.0, 307.0]).len, 94.8262);
}
#[test]
fn aurora_mgkldwnjod() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([862.0, 631.0], [952.0, 605.0]).len, 93.6803);
}
#[test]
fn aurora_wftrssyfux() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([356.0, 124.0], [452.0, 112.0]).len, 97.6132);
}
#[test]
fn aurora_kzmqfyvbis() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([169.0, 531.0], [258.0, 509.0]).len, 93.672);
}
#[test]
fn aurora_mpansjcicm() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([357.0, 722.0], [449.0, 695.0]).len, 95.8801);
}
#[test]
fn aurora_fzereafvdt() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([651.0, 529.0], [678.0, 624.0]).len, 101.655);
}
#[test]
fn aurora_pyibjoxrnk() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([170.0, 648.0], [253.0, 659.0]).len, 100.206);
}
#[test]
fn aurora_fbubcbwnfh() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([122.0, 383.0], [211.0, 346.0]).len, 96.3846);
}
#[test]
fn aurora_txxrppmcnz() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([355.0, 488.0], [436.0, 515.0]).len, 101.788);
}
#[test]
fn aurora_vsfzuuodht() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([828.0, 95.0], [780.0, 130.0]).len, 100.879);
}
#[test]
fn aurora_hhderbqucv() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([67.0, 376.0], [138.0, 434.0]).len, 97.9025);
}
#[test]
fn aurora_dzuvoiwfms() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([625.0, 130.0], [716.0, 90.0]).len, 99.917);
}
#[test]
fn aurora_rwndjrkpxn() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([482.0, 50.0], [433.0, 122.0]).len, 99.7744);
}
#[test]
fn aurora_fgankbiyym() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([150.0, 589.0], [224.0, 516.0]).len, 103.948);
}
#[test]
fn aurora_ulzzpptxyq() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([526.0, 313.0], [437.0, 351.0]).len, 96.7729);
}
#[test]
fn aurora_nwzamkirdn() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([250.0, 656.0], [203.0, 576.0]).len, 107.144);
}
#[test]
fn aurora_bebqddclur() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([824.0, 607.0], [836.0, 512.0]).len, 103.089);
}
#[test]
fn aurora_yareyjhhil() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([859.0, 219.0], [943.0, 278.0]).len, 102.65);
}
#[test]
fn aurora_cjkcxizhit() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([429.0, 487.0], [527.0, 456.0]).len, 102.793);
}
#[test]
fn aurora_gecbtqlxtl() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([827.0, 661.0], [765.0, 726.0]).len, 106.58);
}
#[test]
fn aurora_vajjziclyd() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([979.0, 312.0], [949.0, 266.0]).len, 100.064);
}
#[test]
fn aurora_fsenzvfroa() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([702.0, 392.0], [729.0, 388.0]).len, 100.328);
}
#[test]
fn aurora_skkwzwekvu() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([1023.0, 360.0], [962.0, 280.0]).len, 106.438);
}
#[test]
fn aurora_lsehkokojn() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([516.0, 203.0], [449.0, 261.0]).len, 103.808);
}
#[test]
fn aurora_sgucgpfcsp() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([546.0, 601.0], [466.0, 575.0]).len, 102.607);
}
#[test]
fn aurora_psodbgmapn() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([715.0, 702.0], [632.0, 654.0]).len, 109.622);
}
#[test]
fn aurora_txdbrineht() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([740.0, 244.0], [737.0, 158.0]).len, 105.764);
}
#[test]
fn aurora_hyysdvvygz() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([78.0, 194.0], [173.0, 150.0]).len, 104.695);
}
#[test]
fn aurora_aarliwuurf() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([134.0, 666.0], [187.0, 574.0]).len, 106.195);
}
#[test]
fn aurora_rnbsojpdrf() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([256.0, 719.0], [365.0, 706.0]).len, 110.25);
}
#[test]
fn aurora_svmdiqupkf() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([852.0, 307.0], [835.0, 239.0]).len, 105.822);
}
#[test]
fn aurora_qlcbsbxykf() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([177.0, 420.0], [180.0, 449.0]).len, 107.705);
}
#[test]
fn aurora_ylnairjvce() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([458.0, 269.0], [518.0, 324.0]).len, 109.626);
}
#[test]
fn aurora_ubverbwatu() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([185.0, 186.0], [239.0, 96.0]).len, 105.766);
}
#[test]
fn aurora_vofzcchfqq() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([115.0, 583.0], [229.0, 584.0]).len, 114.005);
}
#[test]
fn aurora_gdsxlgiunu() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([166.0, 631.0], [186.0, 718.0]).len, 108.15);
}
#[test]
fn aurora_szorqxnmlp() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([607.0, 367.0], [501.0, 384.0]).len, 109.317);
}
#[test]
fn aurora_aycxotzzof() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([876.0, 314.0], [775.0, 307.0]).len, 109.238);
}
#[test]
fn aurora_pnyjezhtwp() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([692.0, 269.0], [622.0, 335.0]).len, 107.923);
}
#[test]
fn aurora_zryqgussdv() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([828.0, 515.0], [766.0, 609.0]).len, 112.606);
}
#[test]
fn aurora_awominbujs() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([773.0, 354.0], [861.0, 289.0]).len, 112.932);
}
#[test]
fn aurora_turgmcgrjd() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([99.0, 387.0], [62.0, 436.0]).len, 110.422);
}
#[test]
fn aurora_qvqjlkoljh() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([176.0, 639.0], [294.0, 639.0]).len, 118.0);
}
#[test]
fn aurora_erpailnozn() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([458.0, 405.0], [549.0, 340.0]).len, 111.844);
}
#[test]
fn aurora_rjwajaciue() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([308.0, 226.0], [195.0, 231.0]).len, 113.579);
}
#[test]
fn aurora_tdkaiwxwxp() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([827.0, 456.0], [921.0, 434.0]).len, 117.226);
}
#[test]
fn aurora_rhpcxmntih() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([469.0, 434.0], [455.0, 539.0]).len, 112.804);
}
#[test]
fn aurora_dquhxqnzor() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([135.0, 656.0], [124.0, 613.0]).len, 111.949);
}
#[test]
fn aurora_zzhvcahgus() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([508.0, 504.0], [393.0, 503.0]).len, 116.462);
}
#[test]
fn aurora_azyfmlhtcu() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([515.0, 427.0], [516.0, 316.0]).len, 113.893);
}
#[test]
fn aurora_rifmopjaxf() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([221.0, 260.0], [124.0, 321.0]).len, 114.586);
}
#[test]
fn aurora_wpnchjszie() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([492.0, 553.0], [480.0, 447.0]).len, 115.111);
}
#[test]
fn aurora_rmiigpjpiv() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([265.0, 319.0], [195.0, 357.0]).len, 115.028);
}
#[test]
fn aurora_qhlllcepag() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([550.0, 201.0], [451.0, 261.0]).len, 115.763);
}
#[test]
fn aurora_agmgpimmyd() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([323.0, 128.0], [337.0, 243.0]).len, 115.849);
}
#[test]
fn aurora_ojzlbuotts() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([429.0, 627.0], [555.0, 628.0]).len, 126.004);
}
#[test]
fn aurora_gqnfuqsrko() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([734.0, 569.0], [850.0, 543.0]).len, 118.878);
}
#[test]
fn aurora_nsjqwvtnny() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([700.0, 595.0], [820.0, 612.0]).len, 121.277);
}
#[test]
fn aurora_pujxijqhvm() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([841.0, 523.0], [837.0, 620.0]).len, 118.67);
}
#[test]
fn aurora_uopjuhojoq() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([422.0, 523.0], [484.0, 548.0]).len, 119.616);
}
#[test]
fn aurora_ajefivbaoc() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([556.0, 541.0], [524.0, 626.0]).len, 124.426);
}
#[test]
fn aurora_ozcxwmejxk() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([159.0, 396.0], [186.0, 489.0]).len, 120.74);
}
#[test]
fn aurora_byafswsxrr() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([816.0, 262.0], [918.0, 265.0]).len, 118.635);
}
#[test]
fn aurora_gdawgdekbo() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([340.0, 103.0], [445.0, 151.0]).len, 115.451);
}
#[test]
fn aurora_ggcrkxzifh() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([755.0, 164.0], [852.0, 111.0]).len, 116.292);
}
#[test]
fn aurora_vkkiyfwioj() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([810.0, 524.0], [894.0, 468.0]).len, 120.924);
}
#[test]
fn aurora_ihhcuzdpmw() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([605.0, 561.0], [689.0, 575.0]).len, 123.012);
}
#[test]
fn aurora_kqgwyjzxft() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([371.0, 179.0], [263.0, 125.0]).len, 120.748);
}
#[test]
fn aurora_mswjmooomi() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([298.0, 741.0], [181.0, 708.0]).len, 121.644);
}
#[test]
fn aurora_tprmnqjneg() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([556.0, 247.0], [531.0, 215.0]).len, 123.026);
}
#[test]
fn aurora_wnforrgjle() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([858.0, 121.0], [760.0, 200.0]).len, 126.749);
}
#[test]
fn aurora_lyfeadmujo() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([259.0, 107.0], [333.0, 187.0]).len, 119.329);
}
#[test]
fn aurora_bdjmzmygbb() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([449.0, 626.0], [549.0, 553.0]).len, 124.803);
}
#[test]
fn aurora_jzprfkqntk() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([422.0, 620.0], [315.0, 562.0]).len, 121.709);
}
#[test]
fn aurora_bfhefdojna() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([736.0, 174.0], [623.0, 139.0]).len, 122.716);
}
#[test]
fn aurora_gimybnmbel() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([571.0, 598.0], [450.0, 628.0]).len, 124.733);
}
#[test]
fn aurora_dvwohszeam() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([494.0, 599.0], [604.0, 608.0]).len, 129.666);
}
#[test]
fn aurora_gfkjyhqhbc() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([323.0, 681.0], [236.0, 706.0]).len, 121.637);
}
#[test]
fn aurora_gbhdajvpqi() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([873.0, 564.0], [977.0, 633.0]).len, 124.808);
}
#[test]
fn aurora_ildtmprlca() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([232.0, 79.0], [117.0, 122.0]).len, 123.911);
}
#[test]
fn aurora_sthnslvsbp() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([35.0, 481.0], [40.0, 404.0]).len, 128.897);
}
#[test]
fn aurora_muyqpwaqqs() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([416.0, 451.0], [296.0, 484.0]).len, 125.304);
}
#[test]
fn aurora_bvcsvehkib() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([456.0, 630.0], [563.0, 561.0]).len, 127.318);
}
#[test]
fn aurora_edjminadpx() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([287.0, 538.0], [407.0, 570.0]).len, 124.193);
}
#[test]
fn aurora_lyboxxnaeb() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([433.0, 666.0], [318.0, 709.0]).len, 122.776);
}
#[test]
fn aurora_jfjhfkohcg() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([523.0, 573.0], [497.0, 453.0]).len, 127.196);
}
#[test]
fn aurora_dddyqnsyxh() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([139.0, 270.0], [183.0, 376.0]).len, 130.02);
}
#[test]
fn aurora_sunlqzxvqg() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([229.0, 222.0], [126.0, 139.0]).len, 132.28);
}
#[test]
fn aurora_tbzatpdpnu() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([609.0, 299.0], [508.0, 364.0]).len, 128.118);
}
#[test]
fn aurora_wlokiikgpl() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([345.0, 344.0], [438.0, 349.0]).len, 126.209);
}
#[test]
fn aurora_ufotiimmue() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([432.0, 110.0], [552.0, 149.0]).len, 126.5);
}
#[test]
fn aurora_ykmkdlkxro() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([201.0, 373.0], [64.0, 368.0]).len, 137.091);
}
#[test]
fn aurora_ljanwkehup() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([469.0, 405.0], [564.0, 307.0]).len, 136.488);
}
#[test]
fn aurora_naxeqnbzub() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([765.0, 374.0], [894.0, 350.0]).len, 131.334);
}
#[test]
fn aurora_swnngqeylc() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([567.0, 416.0], [623.0, 347.0]).len, 128.501);
}
#[test]
fn aurora_gnesebntkp() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([982.0, 359.0], [859.0, 311.0]).len, 133.355);
}
#[test]
fn aurora_odvihurnxf() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([825.0, 254.0], [712.0, 187.0]).len, 132.362);
}
#[test]
fn aurora_vmsdkvmsvx() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([177.0, 581.0], [271.0, 556.0]).len, 136.399);
}
#[test]
fn aurora_wifgvplkhq() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([611.0, 477.0], [705.0, 567.0]).len, 136.083);
}
#[test]
fn aurora_ydbovdpuww() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([467.0, 676.0], [598.0, 661.0]).len, 133.475);
}
#[test]
fn aurora_fhswjeowty() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([260.0, 502.0], [291.0, 597.0]).len, 136.976);
}
#[test]
fn aurora_gvpjjylovz() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([805.0, 727.0], [758.0, 635.0]).len, 134.216);
}
#[test]
fn aurora_inywlditvp() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([429.0, 687.0], [342.0, 701.0]).len, 132.358);
}
#[test]
fn aurora_byilghhgke() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([607.0, 40.0], [730.0, 42.0]).len, 133.171);
}
#[test]
fn aurora_huxugnqaiv() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([262.0, 499.0], [376.0, 562.0]).len, 130.287);
}
#[test]
fn aurora_prlabwsbtl() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([473.0, 285.0], [604.0, 326.0]).len, 139.231);
}
#[test]
fn aurora_fhwgiaobpq() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([528.0, 292.0], [572.0, 225.0]).len, 137.677);
}
#[test]
fn aurora_tcvwyqclmc() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([800.0, 249.0], [720.0, 152.0]).len, 138.227);
}
#[test]
fn aurora_cfikcavmja() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([554.0, 686.0], [425.0, 686.0]).len, 135.514);
}
#[test]
fn aurora_stcbqfapoj() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([156.0, 258.0], [293.0, 281.0]).len, 140.492);
}
#[test]
fn aurora_taipitshme() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([404.0, 92.0], [525.0, 154.0]).len, 135.96);
}
#[test]
fn aurora_afklpvvtwd() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([682.0, 532.0], [654.0, 649.0]).len, 134.929);
}
#[test]
fn aurora_yspdpfztjw() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([340.0, 697.0], [306.0, 660.0]).len, 136.33);
}
#[test]
fn aurora_ieutcawfuy() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([688.0, 355.0], [817.0, 324.0]).len, 135.82);
}
#[test]
fn aurora_ckilmljiii() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([524.0, 300.0], [547.0, 375.0]).len, 134.456);
}
#[test]
fn aurora_cudyublrog() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([427.0, 269.0], [551.0, 206.0]).len, 139.086);
}
#[test]
fn aurora_byrhnxgxvn() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([449.0, 208.0], [348.0, 98.0]).len, 149.416);
}
#[test]
fn aurora_qipswkfhzg() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([457.0, 483.0], [366.0, 386.0]).len, 142.206);
}
#[test]
fn aurora_yefendgadg() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([850.0, 365.0], [957.0, 460.0]).len, 144.053);
}
#[test]
fn aurora_owgucrxpol() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([766.0, 149.0], [907.0, 129.0]).len, 142.585);
}
#[test]
fn aurora_qslhveaaum() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([541.0, 247.0], [413.0, 301.0]).len, 140.008);
}
#[test]
fn aurora_ndmyavpiaj() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([825.0, 378.0], [918.0, 469.0]).len, 141.696);
}
#[test]
fn aurora_rkfhmvggbm() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([711.0, 346.0], [831.0, 278.0]).len, 137.928);
}
#[test]
fn aurora_pmyxtbycsu() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([880.0, 500.0], [837.0, 547.0]).len, 138.679);
}
#[test]
fn aurora_tryqsitfim() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([686.0, 581.0], [812.0, 639.0]).len, 138.763);
}
#[test]
fn aurora_bpohiajbeg() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([121.0, 241.0], [39.0, 151.0]).len, 142.817);
}
#[test]
fn aurora_drmxdwofgn() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([334.0, 349.0], [202.0, 375.0]).len, 142.821);
}
#[test]
fn aurora_upvrkibxuc() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([935.0, 610.0], [833.0, 524.0]).len, 143.02);
}
#[test]
fn aurora_zsuanpycwf() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([455.0, 532.0], [406.0, 409.0]).len, 149.544);
}
#[test]
fn aurora_ckzkfphmqn() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([812.0, 191.0], [672.0, 227.0]).len, 144.617);
}
#[test]
fn aurora_ipbsmfnumq() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([683.0, 406.0], [667.0, 317.0]).len, 144.737);
}
#[test]
fn aurora_iyalytemlk() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([486.0, 683.0], [628.0, 681.0]).len, 145.697);
}
#[test]
fn aurora_kcybfsdlnp() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([399.0, 261.0], [308.0, 150.0]).len, 146.233);
}
#[test]
fn aurora_fqpvfqhkvs() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([773.0, 504.0], [903.0, 464.0]).len, 141.756);
}
#[test]
fn aurora_qpjoylikhc() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([321.0, 522.0], [215.0, 581.0]).len, 144.722);
}
#[test]
fn aurora_kjmmnemzip() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([790.0, 142.0], [810.0, 256.0]).len, 147.918);
}
#[test]
fn aurora_suyevxuufz() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([797.0, 248.0], [933.0, 276.0]).len, 147.208);
}
#[test]
fn aurora_ioraxlhmaq() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([769.0, 200.0], [900.0, 138.0]).len, 144.931);
}
#[test]
fn aurora_tyeawyjlkm() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([588.0, 367.0], [497.0, 325.0]).len, 148.565);
}
#[test]
fn aurora_ckkkiairze() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([277.0, 389.0], [386.0, 300.0]).len, 148.544);
}
#[test]
fn aurora_mnmcseqean() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([119.0, 646.0], [233.0, 723.0]).len, 146.179);
}
#[test]
fn aurora_yulhfwexmi() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([579.0, 135.0], [672.0, 236.0]).len, 147.476);
}
#[test]
fn aurora_gcagltcter() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([237.0, 54.0], [203.0, 197.0]).len, 149.81);
}
#[test]
fn aurora_crgwhcfpps() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([185.0, 152.0], [33.0, 158.0]).len, 153.783);
}
#[test]
fn aurora_qdxcigsomi() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([920.0, 444.0], [789.0, 429.0]).len, 149.376);
}
#[test]
fn aurora_tbkgypvemx() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([594.0, 182.0], [742.0, 187.0]).len, 150.47);
}
#[test]
fn aurora_waukevlgmk() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([443.0, 480.0], [389.0, 588.0]).len, 152.957);
}
#[test]
fn aurora_dufegjdlvn() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([882.0, 490.0], [869.0, 369.0]).len, 155.027);
}
#[test]
fn aurora_gvmnqokvuq() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([258.0, 125.0], [340.0, 252.0]).len, 152.023);
}
#[test]
fn aurora_uumizommff() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([236.0, 480.0], [88.0, 454.0]).len, 152.556);
}
#[test]
fn aurora_aeekcmejrt() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([766.0, 715.0], [772.0, 609.0]).len, 155.232);
}
#[test]
fn aurora_gdlbvuvcch() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([820.0, 274.0], [950.0, 349.0]).len, 150.106);
}
#[test]
fn aurora_xtfotuetrh() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([355.0, 563.0], [503.0, 546.0]).len, 152.626);
}
#[test]
fn aurora_mmplkzjtat() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([794.0, 674.0], [928.0, 604.0]).len, 151.182);
}
#[test]
fn aurora_bxvqsxopvd() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([462.0, 255.0], [487.0, 147.0]).len, 151.721);
}
#[test]
fn aurora_ofjaoyhudf() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([272.0, 577.0], [113.0, 582.0]).len, 160.231);
}
#[test]
fn aurora_tylvfpuzea() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([222.0, 644.0], [373.0, 677.0]).len, 160.811);
}
#[test]
fn aurora_fzkysbhbnc() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([335.0, 158.0], [481.0, 202.0]).len, 153.226);
}
#[test]
fn aurora_iafxbbilbv() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([260.0, 312.0], [185.0, 416.0]).len, 158.572);
}
#[test]
fn aurora_apuravukxp() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([582.0, 598.0], [473.0, 673.0]).len, 153.715);
}
#[test]
fn aurora_jldmrfmnye() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([633.0, 195.0], [708.0, 70.0]).len, 158.24);
}
#[test]
fn aurora_hpbdtxerhe() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([184.0, 541.0], [53.0, 559.0]).len, 154.085);
}
#[test]
fn aurora_csflzpmgma() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([121.0, 603.0], [273.0, 567.0]).len, 156.453);
}
#[test]
fn aurora_waydkjqfmg() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([617.0, 500.0], [478.0, 556.0]).len, 157.818);
}
#[test]
fn aurora_ulcyqzxlfb() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([926.0, 461.0], [989.0, 330.0]).len, 158.092);
}
#[test]
fn aurora_wudxgjztcu() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([557.0, 132.0], [412.0, 193.0]).len, 165.026);
}
#[test]
fn aurora_yskzcqvkwy() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([934.0, 627.0], [770.0, 616.0]).len, 164.585);
}
#[test]
fn aurora_otbpqaqhsn() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([305.0, 697.0], [187.0, 711.0]).len, 159.321);
}
#[test]
fn aurora_uleqjidcpi() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([316.0, 373.0], [410.0, 372.0]).len, 157.286);
}
#[test]
fn aurora_xvqbtiwyqo() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([258.0, 627.0], [146.0, 530.0]).len, 157.567);
}
#[test]
fn aurora_wysmtsmwjx() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([836.0, 651.0], [815.0, 492.0]).len, 161.868);
}
#[test]
fn aurora_nnnujvhuxb() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([351.0, 368.0], [348.0, 462.0]).len, 159.461);
}
#[test]
fn aurora_vyggjhuhps() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([206.0, 469.0], [330.0, 573.0]).len, 163.413);
}
#[test]
fn aurora_tkowzrerxd() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([692.0, 544.0], [835.0, 475.0]).len, 158.777);
}
#[test]
fn aurora_llwnorktuo() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([265.0, 149.0], [131.0, 201.0]).len, 161.894);
}
#[test]
fn aurora_prriffetes() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([338.0, 474.0], [213.0, 434.0]).len, 163.459);
}
#[test]
fn aurora_laxqxfshxe() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([979.0, 430.0], [850.0, 441.0]).len, 161.263);
}
#[test]
fn aurora_kleeafrpnr() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([329.0, 193.0], [233.0, 281.0]).len, 159.323);
}
#[test]
fn aurora_hqerrdpmva() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([306.0, 315.0], [421.0, 436.0]).len, 169.07);
}
#[test]
fn aurora_vwkarhogic() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([142.0, 660.0], [232.0, 524.0]).len, 168.439);
}
#[test]
fn aurora_eqkxtkvbyf() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([358.0, 115.0], [212.0, 165.0]).len, 162.386);
}
#[test]
fn aurora_eshtmyfhud() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([308.0, 387.0], [177.0, 478.0]).len, 161.827);
}
#[test]
fn aurora_xvtmsbussb() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([577.0, 373.0], [724.0, 416.0]).len, 159.53);
}
#[test]
fn aurora_gyaakcqubl() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([373.0, 124.0], [292.0, 210.0]).len, 164.841);
}
#[test]
fn aurora_odtdfgsjvx() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([588.0, 113.0], [451.0, 204.0]).len, 166.026);
}
#[test]
fn aurora_wjondiqwvq() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([511.0, 192.0], [357.0, 153.0]).len, 164.475);
}
#[test]
fn aurora_othaoyuuuq() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([86.0, 220.0], [140.0, 338.0]).len, 167.778);
}
#[test]
fn aurora_obmrnrmuuy() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([283.0, 146.0], [271.0, 245.0]).len, 166.83);
}
#[test]
fn aurora_ymybsjffci() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([752.0, 123.0], [592.0, 151.0]).len, 167.203);
}
#[test]
fn aurora_kazdomgnfi() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([249.0, 473.0], [177.0, 616.0]).len, 172.179);
}
#[test]
fn aurora_qfgapevsdm() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([750.0, 343.0], [906.0, 343.0]).len, 164.868);
}
#[test]
fn aurora_wjduuxuurb() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([281.0, 680.0], [155.0, 673.0]).len, 164.082);
}
#[test]
fn aurora_dgwzrvfayb() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([404.0, 581.0], [373.0, 483.0]).len, 167.047);
}
#[test]
fn aurora_wxjqymmbbi() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([855.0, 365.0], [870.0, 480.0]).len, 170.752);
}
#[test]
fn aurora_zpaubznktz() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([858.0, 461.0], [806.0, 610.0]).len, 169.693);
}
#[test]
fn aurora_louohelqoq() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([931.0, 443.0], [1003.0, 299.0]).len, 174.761);
}
#[test]
fn aurora_ddusbhsxci() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([534.0, 473.0], [390.0, 386.0]).len, 168.241);
}
#[test]
fn aurora_vmdgvbuink() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([414.0, 519.0], [570.0, 564.0]).len, 170.573);
}
#[test]
fn aurora_ohnsfrsajz() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([437.0, 393.0], [545.0, 280.0]).len, 171.173);
}
#[test]
fn aurora_onukwjlxuv() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([83.0, 474.0], [169.0, 326.0]).len, 171.883);
}
#[test]
fn aurora_qksugrrjor() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([181.0, 636.0], [23.0, 612.0]).len, 172.545);
}
#[test]
fn aurora_flfvuqtjnc() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([375.0, 604.0], [223.0, 533.0]).len, 171.12);
}
#[test]
fn aurora_wchefbosbb() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([935.0, 612.0], [800.0, 693.0]).len, 167.634);
}
#[test]
fn aurora_qpgscdfaru() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([431.0, 242.0], [279.0, 173.0]).len, 167.413);
}
#[test]
fn aurora_rlzacdsfqi() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([465.0, 295.0], [622.0, 339.0]).len, 169.643);
}
#[test]
fn aurora_mooxaephwb() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([544.0, 660.0], [642.0, 514.0]).len, 175.98);
}
#[test]
fn aurora_sqklvamfbe() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([243.0, 688.0], [240.0, 528.0]).len, 178.875);
}
#[test]
fn aurora_euhsphjzri() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([819.0, 231.0], [650.0, 195.0]).len, 175.312);
}
#[test]
fn aurora_sxvdcdzdde() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([375.0, 484.0], [519.0, 556.0]).len, 171.601);
}
#[test]
fn aurora_hxnotfjeod() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([322.0, 545.0], [480.0, 508.0]).len, 174.304);
}
#[test]
fn aurora_ybsbpnlzlp() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([924.0, 560.0], [856.0, 466.0]).len, 172.538);
}
#[test]
fn aurora_xvlmsmpney() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([579.0, 293.0], [411.0, 311.0]).len, 172.892);
}
#[test]
fn aurora_oveeipzhps() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([334.0, 331.0], [503.0, 339.0]).len, 175.926);
}
#[test]
fn aurora_rghfbdchhm() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([679.0, 354.0], [590.0, 280.0]).len, 172.808);
}
#[test]
fn aurora_ivtajitait() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([551.0, 328.0], [377.0, 315.0]).len, 177.865);
}
#[test]
fn aurora_hxromhllmm() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([890.0, 490.0], [980.0, 446.0]).len, 181.533);
}
#[test]
fn aurora_wdbwlmofbo() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([243.0, 295.0], [101.0, 364.0]).len, 177.181);
}
#[test]
fn aurora_tqfjfzblkp() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([846.0, 438.0], [741.0, 294.0]).len, 181.413);
}
#[test]
fn aurora_epchobnejq() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([766.0, 514.0], [610.0, 596.0]).len, 176.298);
}
#[test]
fn aurora_mxpvpgwagi() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([2.0, 409.0], [174.0, 390.0]).len, 179.584);
}
#[test]
fn aurora_apaxqzecxw() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([35.0, 506.0], [204.0, 453.0]).len, 178.924);
}
#[test]
fn aurora_hhajyviojy() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([276.0, 248.0], [236.0, 357.0]).len, 179.213);
}
#[test]
fn aurora_mvxzzuzesl() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([346.0, 662.0], [189.0, 686.0]).len, 175.221);
}
#[test]
fn aurora_nyvhbgtizf() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([915.0, 136.0], [743.0, 110.0]).len, 176.634);
}
#[test]
fn aurora_rzzyxlqqrv() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([459.0, 488.0], [286.0, 512.0]).len, 181.012);
}
#[test]
fn aurora_ysujusfkpf() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([809.0, 250.0], [904.0, 325.0]).len, 180.298);
}
#[test]
fn aurora_sxgmolypux() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([681.0, 180.0], [844.0, 194.0]).len, 179.713);
}
#[test]
fn aurora_ohlasoanuu() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([863.0, 117.0], [678.0, 100.0]).len, 186.097);
}
#[test]
fn aurora_zhpecekaix() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([332.0, 99.0], [493.0, 55.0]).len, 179.698);
}
#[test]
fn aurora_mmygigqkmw() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([603.0, 188.0], [762.0, 104.0]).len, 179.825);
}
#[test]
fn aurora_ewxdyrkoba() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([650.0, 622.0], [764.0, 530.0]).len, 182.094);
}
#[test]
fn aurora_qilciinwdb() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([255.0, 23.0], [414.0, 108.0]).len, 180.294);
}
#[test]
fn aurora_erakangjkn() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([464.0, 444.0], [589.0, 560.0]).len, 181.807);
}
#[test]
fn aurora_ctyfedjwvo() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([634.0, 294.0], [780.0, 365.0]).len, 181.317);
}
#[test]
fn aurora_flemrgoiyr() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([894.0, 172.0], [725.0, 211.0]).len, 183.489);
}
#[test]
fn aurora_jikbdehbme() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([197.0, 485.0], [39.0, 386.0]).len, 187.699);
}
#[test]
fn aurora_hwgvqnxmcj() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([448.0, 230.0], [609.0, 248.0]).len, 184.641);
}
#[test]
fn aurora_rxzebllpdd() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([168.0, 170.0], [338.0, 235.0]).len, 182.003);
}
#[test]
fn aurora_ehtnzntayc() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([70.0, 352.0], [45.0, 519.0]).len, 183.486);
}
#[test]
fn aurora_gfxscaurru() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([232.0, 643.0], [105.0, 516.0]).len, 189.423);
}
#[test]
fn aurora_nyrodqqfki() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([696.0, 112.0], [565.0, 38.0]).len, 183.043);
}
#[test]
fn aurora_imyygnnzeq() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([481.0, 361.0], [309.0, 330.0]).len, 183.115);
}
#[test]
fn aurora_syrgclpyss() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([248.0, 213.0], [251.0, 53.0]).len, 185.151);
}
#[test]
fn aurora_udniadtbqy() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([245.0, 323.0], [99.0, 228.0]).len, 185.278);
}
#[test]
fn aurora_eudgbflclh() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([175.0, 616.0], [52.0, 509.0]).len, 188.069);
}
#[test]
fn aurora_pwgjrypsib() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([592.0, 376.0], [681.0, 451.0]).len, 192.924);
}
#[test]
fn aurora_csxtvgegti() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([697.0, 675.0], [642.0, 510.0]).len, 191.851);
}
#[test]
fn aurora_npvewchkyh() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([781.0, 198.0], [602.0, 154.0]).len, 188.678);
}
#[test]
fn aurora_unfkuvcctq() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([749.0, 97.0], [621.0, 222.0]).len, 186.518);
}
#[test]
fn aurora_svfbagqiwy() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([322.0, 287.0], [173.0, 328.0]).len, 188.084);
}
#[test]
fn aurora_xcpdsbprtw() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([854.0, 448.0], [823.0, 629.0]).len, 194.695);
}
#[test]
fn aurora_orqxepimbz() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([602.0, 513.0], [678.0, 630.0]).len, 186.046);
}
#[test]
fn aurora_yugkzzxvax() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([747.0, 566.0], [749.0, 669.0]).len, 192.915);
}
#[test]
fn aurora_iirwteshri() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([355.0, 266.0], [463.0, 157.0]).len, 186.596);
}
#[test]
fn aurora_ueapwqlktz() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([147.0, 705.0], [321.0, 638.0]).len, 186.55);
}
#[test]
fn aurora_bpjjqmagxt() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([608.0, 378.0], [471.0, 300.0]).len, 194.326);
}
#[test]
fn aurora_xxerfleigj() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([187.0, 136.0], [345.0, 253.0]).len, 196.664);
}
#[test]
fn aurora_zlvvvwrgbf() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([537.0, 209.0], [371.0, 305.0]).len, 191.76);
}
#[test]
fn aurora_kimfouuegw() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([549.0, 562.0], [735.0, 514.0]).len, 194.016);
}
#[test]
fn aurora_dggkgonzkz() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([33.0, 601.0], [223.0, 623.0]).len, 196.21);
}
#[test]
fn aurora_wmxcmjwqbp() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([475.0, 301.0], [658.0, 251.0]).len, 198.251);
}
#[test]
fn aurora_rkqzmwemod() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([842.0, 670.0], [652.0, 650.0]).len, 193.201);
}
#[test]
fn aurora_ncsnorgdzs() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([103.0, 700.0], [228.0, 565.0]).len, 195.824);
}
#[test]
fn aurora_yaxqxdpibe() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([386.0, 661.0], [524.0, 644.0]).len, 200.087);
}
#[test]
fn aurora_whtujvnffi() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([835.0, 475.0], [889.0, 626.0]).len, 196.045);
}
#[test]
fn aurora_usuxfszlza() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([990.0, 423.0], [819.0, 369.0]).len, 196.748);
}
#[test]
fn aurora_seppswtvqs() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([915.0, 533.0], [978.0, 406.0]).len, 197.576);
}
#[test]
fn aurora_bloctfqvsp() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([794.0, 134.0], [620.0, 51.0]).len, 193.667);
}
#[test]
fn aurora_jatxrvvwmz() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([9.0, 437.0], [160.0, 504.0]).len, 204.523);
}
#[test]
fn aurora_dcxuytluxe() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([88.0, 292.0], [277.0, 242.0]).len, 197.357);
}
#[test]
fn aurora_alguloudjl() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([226.0, 229.0], [33.0, 197.0]).len, 197.895);
}
#[test]
fn aurora_ouxaxfjdjc() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([530.0, 162.0], [726.0, 192.0]).len, 199.664);
}
#[test]
fn aurora_nilnwcdesq() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([508.0, 454.0], [518.0, 633.0]).len, 199.708);
}
#[test]
fn aurora_akabqwstac() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([771.0, 378.0], [628.0, 315.0]).len, 198.242);
}
#[test]
fn aurora_jklntrtvmw() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([792.0, 84.0], [626.0, 188.0]).len, 196.305);
}
#[test]
fn aurora_ogsoxmvtze() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([402.0, 505.0], [597.0, 501.0]).len, 206.097);
}
#[test]
fn aurora_gyxddkagbw() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([978.0, 326.0], [983.0, 455.0]).len, 207.018);
}
#[test]
fn aurora_pcnxofywti() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([304.0, 537.0], [491.0, 472.0]).len, 201.396);
}
#[test]
fn aurora_magdlvmtlw() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([622.0, 698.0], [813.0, 660.0]).len, 199.122);
}
#[test]
fn aurora_fwxncmtqrt() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([389.0, 476.0], [337.0, 316.0]).len, 204.466);
}
#[test]
fn aurora_ihhcmecnyf() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([958.0, 384.0], [877.0, 243.0]).len, 202.756);
}
#[test]
fn aurora_nbslvbnioz() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([133.0, 106.0], [220.0, 220.0]).len, 202.284);
}
#[test]
fn aurora_gxtdgcdmkl() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([681.0, 697.0], [548.0, 605.0]).len, 199.454);
}
#[test]
fn aurora_qruzjuvrop() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([701.0, 426.0], [836.0, 522.0]).len, 198.012);
}
#[test]
fn aurora_rgyiavjlpb() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([160.0, 291.0], [42.0, 434.0]).len, 199.254);
}
#[test]
fn aurora_pogniwmdhe() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([396.0, 590.0], [590.0, 644.0]).len, 202.771);
}
#[test]
fn aurora_ocwpjqdnfx() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([801.0, 446.0], [730.0, 324.0]).len, 206.696);
}
#[test]
fn aurora_vfhaihizzc() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([597.0, 457.0], [414.0, 377.0]).len, 200.115);
}
#[test]
fn aurora_uiifxiztse() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([125.0, 216.0], [298.0, 174.0]).len, 203.715);
}
#[test]
fn aurora_bgnslazahh() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([273.0, 525.0], [288.0, 665.0]).len, 202.155);
}
#[test]
fn aurora_fsxianeqds() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([601.0, 533.0], [715.0, 665.0]).len, 205.821);
}
#[test]
fn aurora_nmolqabxef() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([711.0, 266.0], [874.0, 175.0]).len, 206.216);
}
#[test]
fn aurora_xqsuaaiywd() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([293.0, 249.0], [177.0, 172.0]).len, 201.476);
}
#[test]
fn aurora_iqjvflgfuc() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([444.0, 430.0], [489.0, 614.0]).len, 207.735);
}
#[test]
fn aurora_zndikqahha() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([458.0, 532.0], [258.0, 539.0]).len, 206.98);
}
#[test]
fn aurora_mzzqsmpfkf() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([664.0, 364.0], [875.0, 362.0]).len, 213.235);
}
#[test]
fn aurora_yjaddznrqy() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([653.0, 318.0], [506.0, 194.0]).len, 206.847);
}
#[test]
fn aurora_rhruwfojia() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([847.0, 326.0], [652.0, 330.0]).len, 207.1);
}
#[test]
fn aurora_fkrvwakusg() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([494.0, 280.0], [583.0, 410.0]).len, 208.274);
}
#[test]
fn aurora_pikgbexnrl() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([38.0, 554.0], [200.0, 461.0]).len, 208.278);
}
#[test]
fn aurora_cgkrwjoxjg() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([805.0, 474.0], [687.0, 354.0]).len, 207.139);
}
#[test]
fn aurora_snmzkhtrkn() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([425.0, 77.0], [303.0, 228.0]).len, 211.559);
}
#[test]
fn aurora_jopmuamjov() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([920.0, 301.0], [910.0, 492.0]).len, 210.543);
}
#[test]
fn aurora_irzxocbxhd() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([1018.0, 306.0], [862.0, 434.0]).len, 207.889);
}
#[test]
fn aurora_qljoqblfay() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([796.0, 115.0], [595.0, 105.0]).len, 207.152);
}
#[test]
fn aurora_yucxhvroxm() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([957.0, 321.0], [877.0, 183.0]).len, 211.778);
}
#[test]
fn aurora_chskiesmju() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([418.0, 694.0], [625.0, 670.0]).len, 213.136);
}
#[test]
fn aurora_zphglgnqdc() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([604.0, 732.0], [607.0, 690.0]).len, 212.884);
}
#[test]
fn aurora_vbdowukzgo() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([922.0, 415.0], [864.0, 522.0]).len, 210.044);
}
#[test]
fn aurora_ebvzgvvfzx() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([385.0, 578.0], [567.0, 680.0]).len, 211.439);
}
#[test]
fn aurora_xggqhvqchq() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([213.0, 460.0], [13.0, 521.0]).len, 211.274);
}
#[test]
fn aurora_hwruclbfyh() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([42.0, 588.0], [199.0, 701.0]).len, 213.179);
}
#[test]
fn aurora_safrbxzesr() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([284.0, 85.0], [368.0, 275.0]).len, 209.189);
}
#[test]
fn aurora_ijuhdopnps() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([210.0, 647.0], [232.0, 466.0]).len, 215.503);
}
#[test]
fn aurora_gtpuazfrsa() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([251.0, 231.0], [111.0, 118.0]).len, 210.622);
}
#[test]
fn aurora_eblfrxxjos() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([778.0, 647.0], [788.0, 487.0]).len, 214.56);
}
#[test]
fn aurora_pstvfkbiph() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([357.0, 484.0], [476.0, 576.0]).len, 213.721);
}
#[test]
fn aurora_ibmyemqpac() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([413.0, 177.0], [473.0, 271.0]).len, 214.377);
}
#[test]
fn aurora_bmncusikue() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([164.0, 351.0], [230.0, 415.0]).len, 221.926);
}
#[test]
fn aurora_udalptpzlk() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([960.0, 274.0], [770.0, 370.0]).len, 213.957);
}
#[test]
fn aurora_peikqmurot() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([173.0, 227.0], [187.0, 346.0]).len, 215.753);
}
#[test]
fn aurora_dypavfknkx() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([894.0, 487.0], [789.0, 348.0]).len, 217.319);
}
#[test]
fn aurora_ndmseuspjl() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([225.0, 274.0], [421.0, 219.0]).len, 217.784);
}
#[test]
fn aurora_wzamyfvccv() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([635.0, 584.0], [837.0, 652.0]).len, 214.457);
}
#[test]
fn aurora_xiryixqlyi() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([621.0, 153.0], [526.0, 28.0]).len, 214.572);
}
#[test]
fn aurora_jhmdteuwko() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([881.0, 244.0], [848.0, 129.0]).len, 218.857);
}
#[test]
fn aurora_zlqkbsiflx() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([741.0, 412.0], [664.0, 523.0]).len, 216.44);
}
#[test]
fn aurora_eeswfxrvso() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([407.0, 561.0], [511.0, 666.0]).len, 217.731);
}
#[test]
fn aurora_ujoetfwzih() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([778.0, 507.0], [950.0, 616.0]).len, 218.827);
}
#[test]
fn aurora_lgypnbzzrs() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([495.0, 376.0], [335.0, 371.0]).len, 214.937);
}
#[test]
fn aurora_uygpjldxsj() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([944.0, 333.0], [728.0, 326.0]).len, 221.049);
}
#[test]
fn aurora_fttnqqieng() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([511.0, 389.0], [309.0, 318.0]).len, 220.553);
}
#[test]
fn aurora_gukhfswxut() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([713.0, 537.0], [504.0, 580.0]).len, 217.888);
}
#[test]
fn aurora_gfaskgnmos() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([504.0, 160.0], [603.0, 270.0]).len, 221.895);
}
#[test]
fn aurora_njxxaunldj() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([799.0, 504.0], [954.0, 383.0]).len, 218.98);
}
#[test]
fn aurora_ngiiurancq() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([364.0, 614.0], [255.0, 485.0]).len, 220.263);
}
#[test]
fn aurora_yxziydmfvb() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([365.0, 602.0], [588.0, 600.0]).len, 226.416);
}
#[test]
fn aurora_mnxtwepnok() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([547.0, 281.0], [696.0, 353.0]).len, 219.968);
}
#[test]
fn aurora_jayaadhlhv() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([721.0, 394.0], [539.0, 341.0]).len, 221.279);
}
#[test]
fn aurora_pjskurxahy() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([156.0, 297.0], [318.0, 375.0]).len, 220.32);
}
#[test]
fn aurora_cqfgjjppcz() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([104.0, 374.0], [79.0, 214.0]).len, 227.57);
}
#[test]
fn aurora_zwcxksbvdi() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([459.0, 364.0], [513.0, 181.0]).len, 227.045);
}
#[test]
fn aurora_aifjwihedv() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([422.0, 339.0], [220.0, 267.0]).len, 222.11);
}
#[test]
fn aurora_yhaqvgxwav() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([581.0, 681.0], [482.0, 637.0]).len, 221.812);
}
#[test]
fn aurora_bwqctuhyly() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([723.0, 329.0], [555.0, 221.0]).len, 219.11);
}
#[test]
fn aurora_espckkdsqg() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([438.0, 636.0], [661.0, 594.0]).len, 229.07);
}
#[test]
fn aurora_tauffreyft() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([424.0, 411.0], [634.0, 465.0]).len, 225.474);
}
#[test]
fn aurora_sjzcgcrzue() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([45.0, 416.0], [229.0, 473.0]).len, 223.592);
}
#[test]
fn aurora_fqqgblrrzw() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([198.0, 352.0], [52.0, 534.0]).len, 239.656);
}
#[test]
fn aurora_rpioczuyvy() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([44.0, 429.0], [184.0, 271.0]).len, 226.393);
}
#[test]
fn aurora_oufsziahrm() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([294.0, 324.0], [486.0, 422.0]).len, 234.908);
}
#[test]
fn aurora_fludsrnlei() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([481.0, 366.0], [304.0, 383.0]).len, 224.219);
}
#[test]
fn aurora_yljikuqhub() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([376.0, 611.0], [368.0, 463.0]).len, 230.688);
}
#[test]
fn aurora_slytogqeya() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([738.0, 262.0], [858.0, 135.0]).len, 223.436);
}
#[test]
fn aurora_alybnzbgkd() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([237.0, 603.0], [26.0, 656.0]).len, 223.627);
}
#[test]
fn aurora_mbuhaqutwj() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([609.0, 415.0], [624.0, 452.0]).len, 225.432);
}
#[test]
fn aurora_cmfiuqovdg() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([989.0, 596.0], [838.0, 473.0]).len, 228.117);
}
#[test]
fn aurora_usjyzvwyxx() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([431.0, 164.0], [194.0, 167.0]).len, 239.016);
}
#[test]
fn aurora_sqwqxeqvjf() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([660.0, 726.0], [855.0, 640.0]).len, 229.616);
}
#[test]
fn aurora_ycbkkbjmiy() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([770.0, 639.0], [555.0, 620.0]).len, 227.223);
}
#[test]
fn aurora_qbnlgcckeq() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([797.0, 613.0], [599.0, 508.0]).len, 231.476);
}
#[test]
fn aurora_lllrfrcxst() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([695.0, 661.0], [618.0, 482.0]).len, 235.097);
}
#[test]
fn aurora_lfdexnyfya() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([211.0, 660.0], [89.0, 541.0]).len, 232.407);
}
#[test]
fn aurora_bidhplikvx() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([387.0, 514.0], [300.0, 483.0]).len, 233.101);
}
#[test]
fn aurora_iemimmvkvf() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([147.0, 333.0], [199.0, 244.0]).len, 231.198);
}
#[test]
fn aurora_zmdnxkqjnx() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([446.0, 654.0], [527.0, 564.0]).len, 238.427);
}
#[test]
fn aurora_btjdudqkct() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([862.0, 484.0], [751.0, 638.0]).len, 233.369);
}
#[test]
fn aurora_uvdcaaehfy() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([323.0, 708.0], [90.0, 686.0]).len, 239.059);
}
#[test]
fn aurora_ggniuhwinf() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([423.0, 93.0], [301.0, 274.0]).len, 238.412);
}
#[test]
fn aurora_xxbldfowjc() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([890.0, 296.0], [860.0, 449.0]).len, 237.411);
}
#[test]
fn aurora_gppivsqjfm() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([287.0, 144.0], [505.0, 217.0]).len, 231.767);
}
#[test]
fn aurora_cqovvqzode() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([449.0, 686.0], [308.0, 707.0]).len, 232.008);
}
#[test]
fn aurora_ppcpvjnmar() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([461.0, 307.0], [452.0, 113.0]).len, 242.494);
}
#[test]
fn aurora_xgmfwzovrb() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([231.0, 480.0], [218.0, 354.0]).len, 235.39);
}
#[test]
fn aurora_fxiqegcgta() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([434.0, 156.0], [198.0, 177.0]).len, 242.862);
}
#[test]
fn aurora_ihfivihpwt() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([734.0, 573.0], [530.0, 683.0]).len, 240.421);
}
#[test]
fn aurora_afnodzlkaz() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([384.0, 294.0], [160.0, 295.0]).len, 235.407);
}
#[test]
fn aurora_eisasiyftj() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([367.0, 280.0], [560.0, 160.0]).len, 242.259);
}
#[test]
fn aurora_potglgpuvm() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([932.0, 416.0], [771.0, 539.0]).len, 235.097);
}
#[test]
fn aurora_tsankavheq() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([808.0, 601.0], [575.0, 637.0]).len, 243.909);
}
#[test]
fn aurora_bgvsoyhznr() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([194.0, 347.0], [33.0, 534.0]).len, 247.181);
}
#[test]
fn aurora_pxhuqscmys() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([495.0, 495.0], [729.0, 532.0]).len, 239.689);
}
#[test]
fn aurora_zpbpvrugtn() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([399.0, 647.0], [502.0, 603.0]).len, 239.987);
}
#[test]
fn aurora_ydacvpgpyz() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([714.0, 358.0], [573.0, 212.0]).len, 237.645);
}
#[test]
fn aurora_mhpdlcvfup() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([642.0, 102.0], [848.0, 169.0]).len, 235.78);
}
#[test]
fn aurora_rhbqtybzzu() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([461.0, 292.0], [234.0, 325.0]).len, 239.853);
}
#[test]
fn aurora_samnevxuqt() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([528.0, 141.0], [288.0, 137.0]).len, 244.596);
}
#[test]
fn aurora_fsqkrajrcq() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([699.0, 588.0], [496.0, 684.0]).len, 242.071);
}
#[test]
fn aurora_gsvmwfnlvy() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([571.0, 474.0], [795.0, 527.0]).len, 240.125);
}
#[test]
fn aurora_dbnbpeovzr() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([852.0, 268.0], [964.0, 434.0]).len, 241.389);
}
#[test]
fn aurora_olpzshdbnw() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([591.0, 564.0], [417.0, 671.0]).len, 244.188);
}
#[test]
fn aurora_ttjlpujwcv() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([232.0, 401.0], [301.0, 302.0]).len, 243.265);
}
#[test]
fn aurora_baxijfwdkj() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([55.0, 589.0], [298.0, 616.0]).len, 248.444);
}
#[test]
fn aurora_xnprrzsnpp() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([353.0, 453.0], [164.0, 593.0]).len, 241.667);
}
#[test]
fn aurora_vxqdddzlde() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([623.0, 706.0], [827.0, 626.0]).len, 243.438);
}
#[test]
fn aurora_ryoahkfdhi() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([31.0, 466.0], [16.0, 541.0]).len, 247.582);
}
#[test]
fn aurora_oaubodnqsr() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([745.0, 670.0], [608.0, 684.0]).len, 246.752);
}
#[test]
fn aurora_lxparaftzi() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([244.0, 185.0], [480.0, 186.0]).len, 249.351);
}
#[test]
fn aurora_aaqiwguznu() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([203.0, 511.0], [420.0, 553.0]).len, 244.072);
}
#[test]
fn aurora_rfrszuzeud() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([909.0, 490.0], [973.0, 280.0]).len, 249.973);
}
#[test]
fn aurora_aunbopgvhl() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([148.0, 469.0], [125.0, 240.0]).len, 248.567);
}
#[test]
fn aurora_ewsihfufcl() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([581.0, 293.0], [818.0, 338.0]).len, 245.051);
}
#[test]
fn aurora_mhbvjfoasr() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([241.0, 333.0], [476.0, 328.0]).len, 243.293);
}
#[test]
fn aurora_krgkhlvzlc() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([433.0, 657.0], [199.0, 663.0]).len, 246.176);
}
#[test]
fn aurora_gnzhzexcvc() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([570.0, 33.0], [431.0, 142.0]).len, 245.148);
}
#[test]
fn aurora_ffhwbtuvqz() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([739.0, 514.0], [519.0, 621.0]).len, 252.843);
}
#[test]
fn aurora_kfccganacg() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([461.0, 126.0], [267.0, 258.0]).len, 249.148);
}
#[test]
fn aurora_jrmwponpqq() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([919.0, 153.0], [714.0, 170.0]).len, 247.913);
}
#[test]
fn aurora_nkybmlegoy() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([635.0, 187.0], [880.0, 147.0]).len, 255.253);
}
#[test]
fn aurora_kjdtgowbqo() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([909.0, 465.0], [694.0, 559.0]).len, 243.719);
}
#[test]
fn aurora_imwbomprho() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([532.0, 147.0], [429.0, 225.0]).len, 254.289);
}
#[test]
fn aurora_jvfjdtmmsn() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([218.0, 602.0], [397.0, 583.0]).len, 245.525);
}
#[test]
fn aurora_lqqhqkwpej() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([179.0, 122.0], [425.0, 172.0]).len, 254.369);
}
#[test]
fn aurora_mxgyxhaizm() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([365.0, 691.0], [255.0, 538.0]).len, 250.886);
}
#[test]
fn aurora_jmtrfxrtzf() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([509.0, 454.0], [734.0, 400.0]).len, 248.635);
}
#[test]
fn aurora_gyevmctdms() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([67.0, 330.0], [217.0, 179.0]).len, 256.281);
}
#[test]
fn aurora_setmvsykug() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([154.0, 675.0], [55.0, 496.0]).len, 256.523);
}
#[test]
fn aurora_jyxihrzkfk() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([577.0, 480.0], [398.0, 595.0]).len, 251.883);
}
#[test]
fn aurora_skwjoiigyz() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([47.0, 450.0], [6.0, 554.0]).len, 249.712);
}
#[test]
fn aurora_fdyobkxfat() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([379.0, 526.0], [298.0, 498.0]).len, 250.278);
}
#[test]
fn aurora_nhiewlkamo() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([314.0, 215.0], [555.0, 143.0]).len, 257.637);
}
#[test]
fn aurora_yohiktekko() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([630.0, 253.0], [456.0, 400.0]).len, 251.281);
}
#[test]
fn aurora_fbskiksyuv() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([207.0, 419.0], [377.0, 604.0]).len, 263.3);
}
#[test]
fn aurora_mmazofuyao() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([43.0, 578.0], [89.0, 446.0]).len, 250.51);
}
#[test]
fn aurora_tkojjbmosy() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([470.0, 51.0], [598.0, 165.0]).len, 254.42);
}
#[test]
fn aurora_uopdocidxg() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([791.0, 188.0], [794.0, 338.0]).len, 254.738);
}
#[test]
fn aurora_malibdoigm() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([169.0, 131.0], [72.0, 293.0]).len, 255.546);
}
#[test]
fn aurora_bjwdjlshbh() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([827.0, 418.0], [701.0, 442.0]).len, 256.967);
}
#[test]
fn aurora_zcvivwdqlk() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([848.0, 279.0], [753.0, 232.0]).len, 256.52);
}
#[test]
fn aurora_trixtkgfmh() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([418.0, 547.0], [183.0, 487.0]).len, 256.25);
}
#[test]
fn aurora_bjymvzwebe() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([815.0, 406.0], [944.0, 547.0]).len, 255.274);
}
#[test]
fn aurora_cdzodgenhv() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([807.0, 198.0], [774.0, 396.0]).len, 259.527);
}
#[test]
fn aurora_jmubdazjqg() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([836.0, 209.0], [1014.0, 379.0]).len, 256.155);
}
#[test]
fn aurora_ufjbdiynyu() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([734.0, 407.0], [524.0, 488.0]).len, 257.078);
}
#[test]
fn aurora_vvvpyvsfhs() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([238.0, 421.0], [237.0, 272.0]).len, 255.65);
}
#[test]
fn aurora_ouzvbrmytw() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([497.0, 665.0], [285.0, 702.0]).len, 263.069);
}
#[test]
fn aurora_iqttyctzny() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([364.0, 166.0], [621.0, 147.0]).len, 264.982);
}
#[test]
fn aurora_uxlfgmqizf() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([650.0, 575.0], [746.0, 417.0]).len, 261.928);
}
#[test]
fn aurora_dxjwtgxpzl() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([259.0, 293.0], [169.0, 187.0]).len, 256.412);
}
#[test]
fn aurora_gjbmkotxmi() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([49.0, 536.0], [115.0, 303.0]).len, 264.878);
}
#[test]
fn aurora_imtualmmly() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([883.0, 372.0], [880.0, 174.0]).len, 259.556);
}
#[test]
fn aurora_riaimtjfby() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([866.0, 351.0], [832.0, 513.0]).len, 261.386);
}
#[test]
fn aurora_hvfzyabhlc() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([102.0, 211.0], [359.0, 232.0]).len, 269.746);
}
#[test]
fn aurora_cxbvizozin() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([822.0, 640.0], [746.0, 423.0]).len, 266.339);
}
#[test]
fn aurora_gqhuavgiqi() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([684.0, 70.0], [827.0, 217.0]).len, 261.986);
}
#[test]
fn aurora_wffmmfbmdg() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([419.0, 574.0], [646.0, 558.0]).len, 267.048);
}
#[test]
fn aurora_ypcssfqqro() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([220.0, 511.0], [1.0, 441.0]).len, 264.169);
}
#[test]
fn aurora_hknnkyhqbi() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([787.0, 432.0], [850.0, 350.0]).len, 268.436);
}
#[test]
fn aurora_kpzxfpybvm() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([451.0, 72.0], [603.0, 243.0]).len, 267.156);
}
#[test]
fn aurora_jmertrzipd() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([916.0, 612.0], [662.0, 652.0]).len, 266.9);
}
#[test]
fn aurora_tibndfqzwz() {
    let aurora = aurora_mesh();
    known_failure!(aurora.path([843.0, 478.0], [609.0, 375.0]).len, 262.484);
}
#[test]
fn aurora_wpmqlpdgib() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([391.0, 663.0], [138.0, 695.0]).len, 266.84);
}
#[test]
fn aurora_mxohbkkhly() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([512.0, 164.0], [269.0, 83.0]).len, 260.84);
}
#[test]
fn aurora_npeffrkouh() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([80.0, 160.0], [82.0, 299.0]).len, 264.576);
}
#[test]
fn aurora_padgrfvall() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([280.0, 568.0], [407.0, 601.0]).len, 259.138);
}
#[test]
fn aurora_lcvijlkkst() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([767.0, 429.0], [889.0, 353.0]).len, 274.702);
}
#[test]
fn aurora_bcwduskmvv() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([362.0, 220.0], [125.0, 306.0]).len, 263.792);
}
#[test]
fn aurora_xcozgpbtgx() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([764.0, 203.0], [940.0, 357.0]).len, 269.994);
}
#[test]
fn aurora_jdchuovhhl() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([220.0, 54.0], [398.0, 231.0]).len, 263.402);
}
#[test]
fn aurora_exhrlyfedx() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([67.0, 402.0], [313.0, 330.0]).len, 266.703);
}
#[test]
fn aurora_vfakmpvrmc() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([149.0, 294.0], [408.0, 316.0]).len, 266.903);
}
#[test]
fn aurora_cyfquiytxe() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([725.0, 260.0], [906.0, 127.0]).len, 261.839);
}
#[test]
fn aurora_cngzmfuaux() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([99.0, 464.0], [28.0, 603.0]).len, 269.578);
}
#[test]
fn aurora_fqvemaxjar() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([242.0, 489.0], [492.0, 568.0]).len, 269.65);
}
#[test]
fn aurora_rxvsuagyle() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([525.0, 529.0], [614.0, 686.0]).len, 267.8);
}
#[test]
fn aurora_howasctedu() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([789.0, 106.0], [535.0, 181.0]).len, 268.19);
}
#[test]
fn aurora_xzykmbkqqm() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([325.0, 640.0], [60.0, 685.0]).len, 276.914);
}
#[test]
fn aurora_augvikwqzv() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([443.0, 349.0], [694.0, 355.0]).len, 271.022);
}
#[test]
fn aurora_murlybvyzr() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([817.0, 584.0], [557.0, 605.0]).len, 274.758);
}
#[test]
fn aurora_fzhvaarzdi() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([72.0, 662.0], [17.0, 488.0]).len, 275.481);
}
#[test]
fn aurora_voaxntdrxv() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([411.0, 521.0], [662.0, 453.0]).len, 273.995);
}
#[test]
fn aurora_xuuypshkii() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([322.0, 375.0], [91.0, 425.0]).len, 273.026);
}
#[test]
fn aurora_kxepqwscie() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([220.0, 493.0], [318.0, 693.0]).len, 272.764);
}
#[test]
fn aurora_jfjkzbqoxb() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([78.0, 660.0], [21.0, 482.0]).len, 275.712);
}
#[test]
fn aurora_tebirlsfca() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([247.0, 97.0], [179.0, 255.0]).len, 269.23);
}
#[test]
fn aurora_mvshdkipjw() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([294.0, 499.0], [37.0, 545.0]).len, 272.027);
}
#[test]
fn aurora_xcmdoitzoj() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([468.0, 163.0], [704.0, 196.0]).len, 275.26);
}
#[test]
fn aurora_qoselsdjnv() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([546.0, 427.0], [760.0, 426.0]).len, 272.081);
}
#[test]
fn aurora_oxdqfcggwe() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([940.0, 430.0], [853.0, 231.0]).len, 278.698);
}
#[test]
fn aurora_ufgmvdpfry() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([414.0, 273.0], [157.0, 363.0]).len, 275.684);
}
#[test]
fn aurora_qgizwhcrfj() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([410.0, 452.0], [449.0, 264.0]).len, 278.757);
}
#[test]
fn aurora_wtryjryecz() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([250.0, 83.0], [115.0, 275.0]).len, 275.379);
}
#[test]
fn aurora_ozxbijekiw() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([32.0, 653.0], [264.0, 500.0]).len, 277.908);
}
#[test]
fn aurora_opvgobqvmg() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([196.0, 69.0], [348.0, 272.0]).len, 271.375);
}
#[test]
fn aurora_crorpduhyj() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([704.0, 282.0], [792.0, 264.0]).len, 270.881);
}
#[test]
fn aurora_kqlcubcxcq() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([329.0, 351.0], [528.0, 545.0]).len, 287.643);
}
#[test]
fn aurora_nqzptikqjr() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([373.0, 726.0], [277.0, 602.0]).len, 278.563);
}
#[test]
fn aurora_fvunohwrec() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([565.0, 607.0], [706.0, 396.0]).len, 282.344);
}
#[test]
fn aurora_zjlvmhzlok() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([140.0, 637.0], [174.0, 453.0]).len, 281.129);
}
#[test]
fn aurora_ikxmaakjzk() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([174.0, 592.0], [443.0, 648.0]).len, 277.524);
}
#[test]
fn aurora_ylepobnzvj() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([774.0, 503.0], [624.0, 721.0]).len, 283.334);
}
#[test]
fn aurora_bophkdqwut() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([393.0, 449.0], [149.0, 575.0]).len, 274.612);
}
#[test]
fn aurora_cjosueijzb() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([349.0, 145.0], [482.0, 274.0]).len, 277.004);
}
#[test]
fn aurora_okopsvjllo() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([712.0, 315.0], [435.0, 299.0]).len, 281.968);
}
#[test]
fn aurora_uafzmxgmee() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([606.0, 170.0], [862.0, 174.0]).len, 281.659);
}
#[test]
fn aurora_klivphgiuv() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([808.0, 270.0], [1012.0, 377.0]).len, 283.56);
}
#[test]
fn aurora_uyweujrabw() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([363.0, 237.0], [295.0, 368.0]).len, 282.696);
}
#[test]
fn aurora_ogajplhvlf() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([140.0, 137.0], [205.0, 302.0]).len, 282.889);
}
#[test]
fn aurora_niqoaogkai() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([868.0, 299.0], [700.0, 420.0]).len, 281.845);
}
#[test]
fn aurora_qqtudcgjvc() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([775.0, 420.0], [967.0, 347.0]).len, 286.67);
}
#[test]
fn aurora_soksjqpvdo() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([794.0, 179.0], [878.0, 348.0]).len, 279.989);
}
#[test]
fn aurora_bpvkjcxcbd() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([194.0, 310.0], [183.0, 109.0]).len, 289.481);
}
#[test]
fn aurora_xcrcuqziqu() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([844.0, 199.0], [677.0, 369.0]).len, 284.239);
}
#[test]
fn aurora_mmbieervgi() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([686.0, 382.0], [505.0, 448.0]).len, 280.895);
}
#[test]
fn aurora_owsfmgmnob() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([743.0, 280.0], [783.0, 355.0]).len, 283.718);
}
#[test]
fn aurora_mibovvscqj() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([62.0, 617.0], [324.0, 550.0]).len, 284.318);
}
#[test]
fn aurora_xnzpzjzyka() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([213.0, 346.0], [361.0, 202.0]).len, 286.119);
}
#[test]
fn aurora_ufetrmotfo() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([96.0, 597.0], [145.0, 421.0]).len, 291.28);
}
#[test]
fn aurora_ayagwmopio() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([570.0, 343.0], [573.0, 132.0]).len, 289.533);
}
#[test]
fn aurora_cjvxmcvuxw() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([497.0, 158.0], [762.0, 259.0]).len, 285.367);
}
#[test]
fn aurora_pgwqdfmegr() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([838.0, 248.0], [625.0, 95.0]).len, 283.857);
}
#[test]
fn aurora_vazoushhxp() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([113.0, 478.0], [110.0, 220.0]).len, 287.128);
}
#[test]
fn aurora_jkwyadxent() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([46.0, 518.0], [176.0, 266.0]).len, 289.133);
}
#[test]
fn aurora_mnyrvnctdk() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([655.0, 636.0], [458.0, 493.0]).len, 284.045);
}
#[test]
fn aurora_yjmvmqidwy() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([648.0, 585.0], [900.0, 479.0]).len, 282.311);
}
#[test]
fn aurora_lrzvfaryfy() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([385.0, 339.0], [472.0, 129.0]).len, 294.848);
}
#[test]
fn aurora_hwurtdpbgj() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([629.0, 239.0], [347.0, 210.0]).len, 290.739);
}
#[test]
fn aurora_wghgrftyex() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([259.0, 627.0], [27.0, 487.0]).len, 290.22);
}
#[test]
fn aurora_hhfcjlotyz() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([808.0, 343.0], [553.0, 211.0]).len, 287.153);
}
#[test]
fn aurora_rdgcujbuzs() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([477.0, 124.0], [454.0, 361.0]).len, 295.356);
}
#[test]
fn aurora_xdlybhkpan() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([233.0, 106.0], [492.0, 174.0]).len, 290.251);
}
#[test]
fn aurora_jhlczcidrt() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([441.0, 141.0], [552.0, 346.0]).len, 289.594);
}
#[test]
fn aurora_roinuymksf() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([287.0, 486.0], [281.0, 715.0]).len, 295.692);
}
#[test]
fn aurora_pekyuxhvfm() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([275.0, 25.0], [111.0, 230.0]).len, 291.598);
}
#[test]
fn aurora_eirovbsnhn() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([724.0, 529.0], [867.0, 420.0]).len, 285.903);
}
#[test]
fn aurora_svpnfwlcrn() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([668.0, 353.0], [823.0, 260.0]).len, 288.852);
}
#[test]
fn aurora_sgenpjptla() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([39.0, 164.0], [13.0, 327.0]).len, 291.061);
}
#[test]
fn aurora_lnlkjkvarj() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([496.0, 591.0], [355.0, 623.0]).len, 297.477);
}
#[test]
fn aurora_anjblqruky() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([228.0, 384.0], [349.0, 210.0]).len, 296.628);
}
#[test]
fn aurora_lsirsfxfte() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([878.0, 576.0], [636.0, 695.0]).len, 291.473);
}
#[test]
fn aurora_kidbqynrms() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([749.0, 705.0], [571.0, 700.0]).len, 295.334);
}
#[test]
fn aurora_kavwdtkpcn() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([270.0, 69.0], [17.0, 193.0]).len, 291.812);
}
#[test]
fn aurora_oaugczueuq() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([230.0, 356.0], [281.0, 200.0]).len, 296.23);
}
#[test]
fn aurora_qcyjaslcic() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([139.0, 490.0], [309.0, 618.0]).len, 296.522);
}
#[test]
fn aurora_pppegwvxgu() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([505.0, 149.0], [449.0, 383.0]).len, 298.154);
}
#[test]
fn aurora_dfgcdxcckg() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([51.0, 299.0], [341.0, 313.0]).len, 298.338);
}
#[test]
fn aurora_mylnclcpkm() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([267.0, 376.0], [523.0, 263.0]).len, 293.847);
}
#[test]
fn aurora_kwomjvwvwv() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([392.0, 627.0], [563.0, 550.0]).len, 297.653);
}
#[test]
fn aurora_ezwotwjdbr() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([132.0, 100.0], [405.0, 158.0]).len, 294.013);
}
#[test]
fn aurora_vrizddrgiv() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([660.0, 196.0], [872.0, 289.0]).len, 299.148);
}
#[test]
fn aurora_ypofxhlvrk() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([335.0, 323.0], [510.0, 139.0]).len, 297.536);
}
#[test]
fn aurora_vlfibxxucp() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([77.0, 375.0], [337.0, 265.0]).len, 298.86);
}
#[test]
fn aurora_znzeumxknm() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([698.0, 615.0], [727.0, 409.0]).len, 304.138);
}
#[test]
fn aurora_bejyheayae() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([458.0, 194.0], [166.0, 132.0]).len, 299.568);
}
#[test]
fn aurora_wwaygufzhi() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([631.0, 464.0], [834.0, 614.0]).len, 299.631);
}
#[test]
fn aurora_oshpjqbgox() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([86.0, 565.0], [32.0, 428.0]).len, 298.591);
}
#[test]
fn aurora_srspyyqbol() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([376.0, 719.0], [233.0, 646.0]).len, 302.246);
}
#[test]
fn aurora_tiezgokzni() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([335.0, 611.0], [138.0, 483.0]).len, 302.391);
}
#[test]
fn aurora_lmhilabxqj() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([701.0, 223.0], [424.0, 171.0]).len, 298.38);
}
#[test]
fn aurora_bsqqjnqxis() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([405.0, 134.0], [115.0, 177.0]).len, 306.041);
}
#[test]
fn aurora_vnmexxheld() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([660.0, 614.0], [946.0, 578.0]).len, 300.536);
}
#[test]
fn aurora_ytlcusnlbf() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([216.0, 305.0], [427.0, 485.0]).len, 305.795);
}
#[test]
fn aurora_azponicdya() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([448.0, 395.0], [420.0, 463.0]).len, 309.479);
}
#[test]
fn aurora_zpcvsgyhpy() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([672.0, 103.0], [859.0, 183.0]).len, 303.844);
}
#[test]
fn aurora_nzuprwpejp() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([466.0, 531.0], [257.0, 578.0]).len, 299.773);
}
#[test]
fn aurora_javqjaipis() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([228.0, 97.0], [179.0, 311.0]).len, 303.671);
}
#[test]
fn aurora_whugwqppiv() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([543.0, 277.0], [637.0, 96.0]).len, 301.05);
}
#[test]
fn aurora_nrnquntapn() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([896.0, 175.0], [693.0, 318.0]).len, 306.758);
}
#[test]
fn aurora_soxacrqfza() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([525.0, 654.0], [311.0, 541.0]).len, 304.721);
}
#[test]
fn aurora_rtpcettpyv() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([470.0, 462.0], [700.0, 615.0]).len, 303.386);
}
#[test]
fn aurora_zjsoolmjrz() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([784.0, 734.0], [619.0, 531.0]).len, 308.02);
}
#[test]
fn aurora_oqaythnqmd() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([646.0, 412.0], [892.0, 285.0]).len, 300.29);
}
#[test]
fn aurora_avozuzmibp() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([376.0, 192.0], [588.0, 27.0]).len, 306.702);
}
#[test]
fn aurora_gmunjmxwgc() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([223.0, 151.0], [196.0, 360.0]).len, 304.762);
}
#[test]
fn aurora_erbhrsfkgo() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([246.0, 586.0], [455.0, 560.0]).len, 304.17);
}
#[test]
fn aurora_yhingiykge() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([445.0, 69.0], [645.0, 260.0]).len, 309.947);
}
#[test]
fn aurora_fbnnyuaaba() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([628.0, 501.0], [647.0, 347.0]).len, 307.523);
}
#[test]
fn aurora_uscthqeivb() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([903.0, 353.0], [711.0, 438.0]).len, 311.782);
}
#[test]
fn aurora_sslanvfxuo() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([62.0, 552.0], [290.0, 664.0]).len, 308.318);
}
#[test]
fn aurora_uxivdabzox() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([491.0, 536.0], [798.0, 546.0]).len, 313.495);
}
#[test]
fn aurora_ryazwyukqt() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([142.0, 142.0], [452.0, 190.0]).len, 314.74);
}
#[test]
fn aurora_btjeeutyvb() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([698.0, 188.0], [470.0, 260.0]).len, 312.797);
}
#[test]
fn aurora_ihtveuohqf() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([197.0, 84.0], [451.0, 235.0]).len, 311.626);
}
#[test]
fn aurora_ufneapvamx() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([398.0, 245.0], [97.0, 178.0]).len, 312.181);
}
#[test]
fn aurora_tllfstxsrw() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([721.0, 370.0], [509.0, 231.0]).len, 305.99);
}
#[test]
fn aurora_lsxeznrghg() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([129.0, 670.0], [73.0, 448.0]).len, 312.338);
}
#[test]
fn aurora_jalljjtoyq() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([778.0, 524.0], [516.0, 659.0]).len, 317.78);
}
#[test]
fn aurora_lnnctsnyst() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([259.0, 78.0], [195.0, 271.0]).len, 313.106);
}
#[test]
fn aurora_lyqxkeqapq() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([255.0, 572.0], [66.0, 395.0]).len, 318.313);
}
#[test]
fn aurora_oxgakrlxgd() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([767.0, 610.0], [483.0, 508.0]).len, 315.955);
}
#[test]
fn aurora_tparsumemw() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([63.0, 333.0], [286.0, 504.0]).len, 316.434);
}
#[test]
fn aurora_norlvrfaqy() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([346.0, 263.0], [45.0, 335.0]).len, 313.267);
}
#[test]
fn aurora_udtbeuscnm() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([336.0, 660.0], [174.0, 498.0]).len, 314.05);
}
#[test]
fn aurora_idpwqwcqgs() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([696.0, 248.0], [440.0, 164.0]).len, 309.947);
}
#[test]
fn aurora_znmodhwibs() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([212.0, 225.0], [509.0, 153.0]).len, 311.711);
}
#[test]
fn aurora_kedgnkebev() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([486.0, 397.0], [565.0, 167.0]).len, 313.135);
}
#[test]
fn aurora_bxzjlwqvzm() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([380.0, 307.0], [258.0, 509.0]).len, 318.701);
}
#[test]
fn aurora_bmljnilxik() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([141.0, 366.0], [337.0, 174.0]).len, 314.686);
}
#[test]
fn aurora_cuhtszzqwx() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([428.0, 265.0], [329.0, 220.0]).len, 316.204);
}
#[test]
fn aurora_ywyxglgxim() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([716.0, 345.0], [494.0, 166.0]).len, 316.741);
}
#[test]
fn aurora_uhextmqioe() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([193.0, 100.0], [501.0, 56.0]).len, 315.439);
}
#[test]
fn aurora_hfwicqfqmg() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([662.0, 447.0], [711.0, 697.0]).len, 321.433);
}
#[test]
fn aurora_qoulxmmbmp() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([207.0, 436.0], [378.0, 640.0]).len, 318.014);
}
#[test]
fn aurora_fqewimqmaq() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([284.0, 553.0], [599.0, 539.0]).len, 323.287);
}
#[test]
fn aurora_zvfzikeltk() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([448.0, 244.0], [559.0, 18.0]).len, 315.571);
}
#[test]
fn aurora_xekvssszil() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([716.0, 363.0], [420.0, 347.0]).len, 317.424);
}
#[test]
fn aurora_zooyuvgips() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([558.0, 678.0], [243.0, 660.0]).len, 323.434);
}
#[test]
fn aurora_charvhxtoc() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([59.0, 514.0], [233.0, 298.0]).len, 319.251);
}
#[test]
fn aurora_vumqtkpmtb() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([117.0, 393.0], [3.0, 174.0]).len, 319.349);
}
#[test]
fn aurora_nufildkjoo() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([550.0, 451.0], [807.0, 422.0]).len, 320.221);
}
#[test]
fn aurora_jhmaseaksu() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([737.0, 683.0], [583.0, 533.0]).len, 325.535);
}
#[test]
fn aurora_azbomkipvo() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([235.0, 561.0], [394.0, 546.0]).len, 317.456);
}
#[test]
fn aurora_qmfoeptbkc() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([376.0, 330.0], [294.0, 218.0]).len, 317.698);
}
#[test]
fn aurora_issdngxgmr() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([324.0, 343.0], [392.0, 569.0]).len, 327.473);
}
#[test]
fn aurora_njsaoqykrp() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([283.0, 537.0], [597.0, 541.0]).len, 324.725);
}
#[test]
fn aurora_yjlrnmnvhe() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([969.0, 274.0], [814.0, 469.0]).len, 331.225);
}
#[test]
fn aurora_trdtnyywws() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([598.0, 474.0], [791.0, 322.0]).len, 321.992);
}
#[test]
fn aurora_geugcabvsz() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([461.0, 601.0], [346.0, 720.0]).len, 327.48);
}
#[test]
fn aurora_frqymazdcv() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([101.0, 670.0], [349.0, 581.0]).len, 325.527);
}
#[test]
fn aurora_ddidhjstem() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([93.0, 696.0], [318.0, 528.0]).len, 325.764);
}
#[test]
fn aurora_vgwwvizzuk() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([256.0, 703.0], [28.0, 595.0]).len, 330.379);
}
#[test]
fn aurora_yiwwyrauxy() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([539.0, 686.0], [246.0, 558.0]).len, 323.091);
}
#[test]
fn aurora_mewetxaqfy() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([648.0, 161.0], [334.0, 136.0]).len, 324.917);
}
#[test]
fn aurora_plyiuhywaw() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([925.0, 344.0], [619.0, 328.0]).len, 326.091);
}
#[test]
fn aurora_pxgfdjnqzt() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([143.0, 438.0], [431.0, 326.0]).len, 323.525);
}
#[test]
fn aurora_vcmjykmfui() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([757.0, 114.0], [899.0, 287.0]).len, 328.351);
}
#[test]
fn aurora_hgzvmrcroz() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([22.0, 158.0], [308.0, 292.0]).len, 328.302);
}
#[test]
fn aurora_iejvnoeinr() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([128.0, 550.0], [117.0, 323.0]).len, 325.882);
}
#[test]
fn aurora_ngqyeaqnfl() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([362.0, 78.0], [669.0, 75.0]).len, 323.802);
}
#[test]
fn aurora_mktjereeam() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([415.0, 409.0], [316.0, 312.0]).len, 327.931);
}
#[test]
fn aurora_efoatorlwu() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([591.0, 502.0], [845.0, 706.0]).len, 329.833);
}
#[test]
fn aurora_qgkulnjdbv() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([685.0, 530.0], [725.0, 340.0]).len, 329.019);
}
#[test]
fn aurora_angjihnjpv() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([371.0, 566.0], [53.0, 509.0]).len, 329.439);
}
#[test]
fn aurora_qhvabyirps() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([74.0, 399.0], [182.0, 155.0]).len, 331.16);
}
#[test]
fn aurora_oagdugnvxg() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([655.0, 587.0], [941.0, 461.0]).len, 325.09);
}
#[test]
fn aurora_whvnywehzq() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([286.0, 360.0], [184.0, 171.0]).len, 331.889);
}
#[test]
fn aurora_olgwxibopx() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([58.0, 133.0], [147.0, 386.0]).len, 334.082);
}
#[test]
fn aurora_nksdlfxbez() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([857.0, 650.0], [573.0, 491.0]).len, 330.302);
}
#[test]
fn aurora_hwumgijkjc() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([206.0, 102.0], [208.0, 335.0]).len, 331.32);
}
#[test]
fn aurora_vtvronncxi() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([380.0, 564.0], [98.0, 535.0]).len, 329.336);
}
#[test]
fn aurora_twzukzhmyr() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([507.0, 311.0], [653.0, 96.0]).len, 330.127);
}
#[test]
fn aurora_mpvebihcfb() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([113.0, 463.0], [398.0, 333.0]).len, 333.927);
}
#[test]
fn aurora_ynixsmxuix() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([233.0, 184.0], [329.0, 339.0]).len, 328.142);
}
#[test]
fn aurora_cbwhueywor() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([19.0, 505.0], [318.0, 371.0]).len, 333.661);
}
#[test]
fn aurora_cjuqocryyb() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([1015.0, 445.0], [693.0, 443.0]).len, 332.103);
}
#[test]
fn aurora_mrbodbbphr() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([374.0, 334.0], [198.0, 521.0]).len, 332.168);
}
#[test]
fn aurora_vnfbetolus() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([284.0, 248.0], [493.0, 329.0]).len, 335.657);
}
#[test]
fn aurora_mznlzuaxia() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([593.0, 661.0], [279.0, 574.0]).len, 335.327);
}
#[test]
fn aurora_wdbxwodbus() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([862.0, 397.0], [545.0, 383.0]).len, 331.306);
}
#[test]
fn aurora_pjtseactxj() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([401.0, 543.0], [174.0, 534.0]).len, 335.188);
}
#[test]
fn aurora_ozaipjacwk() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([652.0, 449.0], [460.0, 680.0]).len, 337.743);
}
#[test]
fn aurora_ekncwmmyna() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([749.0, 417.0], [895.0, 235.0]).len, 333.643);
}
#[test]
fn aurora_tdlooarlqt() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([664.0, 239.0], [434.0, 274.0]).len, 332.787);
}
#[test]
fn aurora_kkaetewqkn() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([468.0, 577.0], [789.0, 489.0]).len, 335.683);
}
#[test]
fn aurora_fpwnllzzbo() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([616.0, 62.0], [306.0, 51.0]).len, 330.974);
}
#[test]
fn aurora_amqckibmiw() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([696.0, 110.0], [826.0, 280.0]).len, 341.174);
}
#[test]
fn aurora_lweucgqucf() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([73.0, 136.0], [164.0, 386.0]).len, 341.101);
}
#[test]
fn aurora_zwpxlprcda() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([477.0, 590.0], [335.0, 752.0]).len, 337.186);
}
#[test]
fn aurora_mucbiwvehk() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([438.0, 499.0], [632.0, 671.0]).len, 336.215);
}
#[test]
fn aurora_idhlqhrdvk() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([253.0, 339.0], [572.0, 247.0]).len, 336.527);
}
#[test]
fn aurora_yrqagrvvof() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([512.0, 412.0], [417.0, 188.0]).len, 337.467);
}
#[test]
fn aurora_hzojitfjrq() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([433.0, 197.0], [132.0, 319.0]).len, 338.677);
}
#[test]
fn aurora_wtvguskmky() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([420.0, 317.0], [205.0, 504.0]).len, 335.174);
}
#[test]
fn aurora_qsaycaoddd() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([427.0, 460.0], [288.0, 265.0]).len, 340.82);
}
#[test]
fn aurora_qcjonchstl() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([502.0, 497.0], [181.0, 462.0]).len, 339.457);
}
#[test]
fn aurora_mhundusdxe() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([808.0, 470.0], [483.0, 521.0]).len, 342.358);
}
#[test]
fn aurora_qcfffdrleh() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([406.0, 464.0], [95.0, 454.0]).len, 340.502);
}
#[test]
fn aurora_phiyobzonb() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([343.0, 254.0], [566.0, 263.0]).len, 346.188);
}
#[test]
fn aurora_guymjezupv() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([903.0, 466.0], [601.0, 604.0]).len, 338.366);
}
#[test]
fn aurora_qducrnjwju() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([554.0, 575.0], [866.0, 442.0]).len, 343.438);
}
#[test]
fn aurora_qtefvaizap() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([767.0, 303.0], [473.0, 143.0]).len, 340.131);
}
#[test]
fn aurora_yompdwykvz() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([277.0, 318.0], [13.0, 529.0]).len, 348.338);
}
#[test]
fn aurora_ptofqarmvg() {
    let aurora = aurora_mesh();
    known_failure!(aurora.path([752.0, 511.0], [639.0, 359.0]).len, 339.859);
}
#[test]
fn aurora_yesmsnslag() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([702.0, 215.0], [790.0, 391.0]).len, 346.261);
}
#[test]
fn aurora_mrwdjialib() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([539.0, 151.0], [764.0, 326.0]).len, 339.301);
}
#[test]
fn aurora_anjelaqwss() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([878.0, 118.0], [774.0, 301.0]).len, 348.095);
}
#[test]
fn aurora_roghkhrnck() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([341.0, 605.0], [34.0, 572.0]).len, 345.946);
}
#[test]
fn aurora_alpleckplh() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([464.0, 361.0], [463.0, 483.0]).len, 345.542);
}
#[test]
fn aurora_wqhutagxzn() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([17.0, 156.0], [316.0, 308.0]).len, 347.137);
}
#[test]
fn aurora_ppjzgxwnno() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([100.0, 274.0], [431.0, 299.0]).len, 346.763);
}
#[test]
fn aurora_oikgcsehhn() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([607.0, 216.0], [453.0, 354.0]).len, 351.466);
}
#[test]
fn aurora_zvclqlstgt() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([305.0, 89.0], [268.0, 342.0]).len, 351.094);
}
#[test]
fn aurora_hsxuizlkjv() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([763.0, 492.0], [442.0, 462.0]).len, 344.025);
}
#[test]
fn aurora_iseothtuva() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([950.0, 365.0], [640.0, 428.0]).len, 357.351);
}
#[test]
fn aurora_gcbtjqhaop() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([459.0, 669.0], [794.0, 658.0]).len, 347.926);
}
#[test]
fn aurora_ejtpmnxznz() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([776.0, 592.0], [715.0, 357.0]).len, 348.08);
}
#[test]
fn aurora_mwamhgvtoh() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([876.0, 172.0], [647.0, 366.0]).len, 349.325);
}
#[test]
fn aurora_qyxwxoksbh() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([579.0, 360.0], [604.0, 540.0]).len, 348.051);
}
#[test]
fn aurora_mdnhicydzk() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([98.0, 307.0], [142.0, 93.0]).len, 353.399);
}
#[test]
fn aurora_xnfjqycaqj() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([453.0, 229.0], [761.0, 248.0]).len, 349.701);
}
#[test]
fn aurora_tnhrtygnzp() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([52.0, 178.0], [389.0, 126.0]).len, 349.761);
}
#[test]
fn aurora_usskbbqofg() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([425.0, 530.0], [159.0, 586.0]).len, 345.464);
}
#[test]
fn aurora_wlaxdjtekq() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([197.0, 275.0], [463.0, 490.0]).len, 354.4);
}
#[test]
fn aurora_ljhlymcwtl() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([634.0, 327.0], [389.0, 133.0]).len, 355.084);
}
#[test]
fn aurora_vpigirwrbo() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([131.0, 159.0], [112.0, 394.0]).len, 350.785);
}
#[test]
fn aurora_haorxdvaai() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([416.0, 525.0], [150.0, 462.0]).len, 350.559);
}
#[test]
fn aurora_juspgdbjhx() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([669.0, 377.0], [633.0, 624.0]).len, 356.228);
}
#[test]
fn aurora_ljpunlrnuf() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([371.0, 570.0], [128.0, 710.0]).len, 354.963);
}
#[test]
fn aurora_phaqknpkzq() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([402.0, 309.0], [97.0, 465.0]).len, 354.542);
}
#[test]
fn aurora_achfqgzool() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([380.0, 83.0], [514.0, 351.0]).len, 357.85);
}
#[test]
fn aurora_rqvamacvmf() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([370.0, 162.0], [112.0, 360.0]).len, 354.594);
}
#[test]
fn aurora_iprimjicvn() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([8.0, 649.0], [331.0, 524.0]).len, 353.463);
}
#[test]
fn aurora_iezyflfgas() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([224.0, 240.0], [115.0, 430.0]).len, 350.03);
}
#[test]
fn aurora_iilfaoykjr() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([771.0, 202.0], [433.0, 190.0]).len, 357.452);
}
#[test]
fn aurora_wzsmrjxgil() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([837.0, 330.0], [488.0, 272.0]).len, 358.773);
}
#[test]
fn aurora_vnocfgkxvu() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([248.0, 269.0], [120.0, 121.0]).len, 352.39);
}
#[test]
fn aurora_cnztgbiejs() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([561.0, 659.0], [250.0, 486.0]).len, 356.431);
}
#[test]
fn aurora_zjmolawwyv() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([604.0, 690.0], [279.0, 568.0]).len, 355.942);
}
#[test]
fn aurora_raykqzutgw() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([333.0, 632.0], [50.0, 477.0]).len, 355.498);
}
#[test]
fn aurora_csiqvmdsjl() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([638.0, 259.0], [326.0, 101.0]).len, 357.855);
}
#[test]
fn aurora_ffgilzrrmq() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([535.0, 248.0], [725.0, 169.0]).len, 365.636);
}
#[test]
fn aurora_tnbunpbmru() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([560.0, 612.0], [648.0, 358.0]).len, 358.285);
}
#[test]
fn aurora_hpurooejad() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([321.0, 732.0], [320.0, 551.0]).len, 357.478);
}
#[test]
fn aurora_ytiynrhwtb() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([119.0, 387.0], [279.0, 633.0]).len, 359.545);
}
#[test]
fn aurora_acltqnghii() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([696.0, 68.0], [361.0, 110.0]).len, 356.232);
}
#[test]
fn aurora_jqdydzyuil() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([877.0, 410.0], [715.0, 631.0]).len, 363.353);
}
#[test]
fn aurora_vxdiyotubc() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([529.0, 547.0], [194.0, 496.0]).len, 358.741);
}
#[test]
fn aurora_llcexhdxfu() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([822.0, 431.0], [734.0, 208.0]).len, 363.901);
}
#[test]
fn aurora_gviohfdbes() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([825.0, 387.0], [700.0, 258.0]).len, 361.149);
}
#[test]
fn aurora_gouwtwkhta() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([77.0, 210.0], [447.0, 212.0]).len, 373.266);
}
#[test]
fn aurora_ebacikoopq() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([907.0, 431.0], [632.0, 599.0]).len, 361.269);
}
#[test]
fn aurora_tmvdzexkkh() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([602.0, 108.0], [690.0, 324.0]).len, 361.726);
}
#[test]
fn aurora_kwkwzmtwfx() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([179.0, 623.0], [320.0, 371.0]).len, 364.53);
}
#[test]
fn aurora_apqyphcasq() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([790.0, 660.0], [446.0, 585.0]).len, 361.427);
}
#[test]
fn aurora_vpfdohhmjd() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([393.0, 281.0], [369.0, 545.0]).len, 365.689);
}
#[test]
fn aurora_mmpbfbgcow() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([292.0, 552.0], [278.0, 303.0]).len, 374.587);
}
#[test]
fn aurora_qjwkhktzrx() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([484.0, 437.0], [614.0, 685.0]).len, 369.36);
}
#[test]
fn aurora_nwjghziagh() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([937.0, 378.0], [725.0, 156.0]).len, 365.721);
}
#[test]
fn aurora_vzzphysvgy() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([499.0, 497.0], [200.0, 350.0]).len, 368.583);
}
#[test]
fn aurora_viuefkstzx() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([580.0, 112.0], [262.0, 263.0]).len, 368.233);
}
#[test]
fn aurora_wihdgzorsv() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([450.0, 580.0], [156.0, 439.0]).len, 360.804);
}
#[test]
fn aurora_hwofeoudig() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([400.0, 335.0], [47.0, 306.0]).len, 368.488);
}
#[test]
fn aurora_essxnkpwxp() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([529.0, 188.0], [832.0, 220.0]).len, 364.854);
}
#[test]
fn aurora_ftspmnmcgj() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([161.0, 373.0], [167.0, 634.0]).len, 375.213);
}
#[test]
fn aurora_xxgjggbpwb() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([473.0, 600.0], [251.0, 632.0]).len, 366.017);
}
#[test]
fn aurora_nojqjztplx() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([118.0, 430.0], [251.0, 211.0]).len, 368.427);
}
#[test]
fn aurora_voufgwotaf() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([430.0, 137.0], [272.0, 368.0]).len, 369.794);
}
#[test]
fn aurora_sbvfxqonkg() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([590.0, 516.0], [935.0, 633.0]).len, 369.327);
}
#[test]
fn aurora_ydixhgjmsw() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([338.0, 374.0], [5.0, 413.0]).len, 368.801);
}
#[test]
fn aurora_izselsxgeb() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([621.0, 479.0], [875.0, 360.0]).len, 374.571);
}
#[test]
fn aurora_irbuidvqrj() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([482.0, 271.0], [240.0, 226.0]).len, 376.014);
}
#[test]
fn aurora_knwdlqrvyl() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([613.0, 200.0], [261.0, 188.0]).len, 375.463);
}
#[test]
fn aurora_ieftvugvga() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([2.0, 362.0], [119.0, 171.0]).len, 366.018);
}
#[test]
fn aurora_xukgkjnzgs() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([793.0, 387.0], [807.0, 594.0]).len, 369.769);
}
#[test]
fn aurora_cpeiezblfr() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([276.0, 275.0], [535.0, 287.0]).len, 367.206);
}
#[test]
fn aurora_zcjqzpgrjv() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([483.0, 367.0], [314.0, 303.0]).len, 368.626);
}
#[test]
fn aurora_ensmohdwrm() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([767.0, 729.0], [437.0, 667.0]).len, 373.482);
}
#[test]
fn aurora_umudsxxmdm() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([871.0, 361.0], [522.0, 298.0]).len, 377.479);
}
#[test]
fn aurora_omvvgszwix() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([121.0, 645.0], [44.0, 396.0]).len, 374.455);
}
#[test]
fn aurora_cjmjbfaujh() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([801.0, 328.0], [446.0, 400.0]).len, 372.177);
}
#[test]
fn aurora_mmsvkewltn() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([540.0, 460.0], [785.0, 332.0]).len, 369.9);
}
#[test]
fn aurora_wmkhuagmen() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([894.0, 620.0], [555.0, 477.0]).len, 370.948);
}
#[test]
fn aurora_jbwqzqhdyx() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([653.0, 285.0], [662.0, 221.0]).len, 369.702);
}
#[test]
fn aurora_zalpgvcsch() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([17.0, 607.0], [383.0, 668.0]).len, 380.292);
}
#[test]
fn aurora_wogewrcmyb() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([463.0, 356.0], [365.0, 263.0]).len, 379.594);
}
#[test]
fn aurora_vcvfwiltld() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([647.0, 585.0], [344.0, 510.0]).len, 376.343);
}
#[test]
fn aurora_whydcjsubo() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([313.0, 176.0], [141.0, 435.0]).len, 376.441);
}
#[test]
fn aurora_fpzmsqmbjt() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([905.0, 377.0], [665.0, 555.0]).len, 376.086);
}
#[test]
fn aurora_fnmdyulwcj() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([850.0, 663.0], [471.0, 636.0]).len, 385.347);
}
#[test]
fn aurora_stossobsbf() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([183.0, 555.0], [264.0, 354.0]).len, 378.811);
}
#[test]
fn aurora_kpxfrjhwyl() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([382.0, 440.0], [730.0, 401.0]).len, 374.328);
}
#[test]
fn aurora_takqsdwxvp() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([294.0, 284.0], [43.0, 505.0]).len, 380.548);
}
#[test]
fn aurora_sknomujmio() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([407.0, 490.0], [515.0, 363.0]).len, 376.821);
}
#[test]
fn aurora_vpprmuvykg() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([338.0, 163.0], [712.0, 108.0]).len, 381.264);
}
#[test]
fn aurora_cfstxnwmqj() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([990.0, 295.0], [747.0, 513.0]).len, 378.891);
}
#[test]
fn aurora_voldesjmxz() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([682.0, 149.0], [344.0, 256.0]).len, 384.67);
}
#[test]
fn aurora_gxdlfbyigz() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([711.0, 404.0], [688.0, 683.0]).len, 387.244);
}
#[test]
fn aurora_hgyrkrblpp() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([613.0, 393.0], [968.0, 384.0]).len, 377.47);
}
#[test]
fn aurora_lkktbdefcd() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([754.0, 405.0], [977.0, 285.0]).len, 387.29);
}
#[test]
fn aurora_gemvkbrsif() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([776.0, 308.0], [614.0, 185.0]).len, 380.363);
}
#[test]
fn aurora_tlnldosnpw() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([483.0, 656.0], [345.0, 492.0]).len, 378.174);
}
#[test]
fn aurora_rcwfxkdfwj() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([27.0, 434.0], [243.0, 234.0]).len, 378.012);
}
#[test]
fn aurora_gqlcojuhco() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([411.0, 412.0], [733.0, 602.0]).len, 377.631);
}
#[test]
fn aurora_kqntgmgfig() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([798.0, 401.0], [726.0, 152.0]).len, 383.313);
}
#[test]
fn aurora_xdkhthlfed() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([187.0, 434.0], [526.0, 545.0]).len, 383.499);
}
#[test]
fn aurora_jvcbjpllxq() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([343.0, 101.0], [99.0, 322.0]).len, 383.827);
}
#[test]
fn aurora_pvugsmggnw() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([137.0, 504.0], [502.0, 569.0]).len, 386.451);
}
#[test]
fn aurora_olwpxqchpt() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([215.0, 368.0], [248.0, 614.0]).len, 388.117);
}
#[test]
fn aurora_uawlvtvoat() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([65.0, 583.0], [162.0, 317.0]).len, 386.419);
}
#[test]
fn aurora_fdiedpjwfu() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([550.0, 269.0], [928.0, 278.0]).len, 386.758);
}
#[test]
fn aurora_ajmmbbfwct() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([324.0, 536.0], [213.0, 304.0]).len, 384.076);
}
#[test]
fn aurora_hwuqskpplh() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([627.0, 318.0], [671.0, 175.0]).len, 385.533);
}
#[test]
fn aurora_bxvmmiuzec() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([318.0, 388.0], [444.0, 238.0]).len, 385.422);
}
#[test]
fn aurora_tchftoupti() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([361.0, 568.0], [49.0, 362.0]).len, 387.463);
}
#[test]
fn aurora_xnxlysmcul() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([14.0, 331.0], [45.0, 581.0]).len, 384.823);
}
#[test]
fn aurora_zaqaijtyvi() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([116.0, 371.0], [285.0, 120.0]).len, 389.062);
}
#[test]
fn aurora_qkbkwdjhmy() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([339.0, 686.0], [394.0, 430.0]).len, 391.129);
}
#[test]
fn aurora_glalvgnesb() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([475.0, 444.0], [129.0, 291.0]).len, 386.706);
}
#[test]
fn aurora_aenyfufnga() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([8.0, 531.0], [343.0, 649.0]).len, 382.428);
}
#[test]
fn aurora_coldbdroou() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([743.0, 239.0], [747.0, 367.0]).len, 389.112);
}
#[test]
fn aurora_tsvemmtybt() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([30.0, 459.0], [217.0, 171.0]).len, 390.319);
}
#[test]
fn aurora_qyuwqtuxnf() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([143.0, 415.0], [324.0, 142.0]).len, 390.041);
}
#[test]
fn aurora_jjuryefali() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([386.0, 300.0], [382.0, 569.0]).len, 391.428);
}
#[test]
fn aurora_tnofmmoxco() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([852.0, 365.0], [750.0, 220.0]).len, 386.869);
}
#[test]
fn aurora_lkzarnofht() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([625.0, 138.0], [252.0, 79.0]).len, 384.961);
}
#[test]
fn aurora_usgwtjldfa() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([637.0, 346.0], [829.0, 584.0]).len, 389.616);
}
#[test]
fn aurora_zksderbbks() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([506.0, 329.0], [901.0, 303.0]).len, 398.747);
}
#[test]
fn aurora_ibmahovdrn() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([482.0, 553.0], [285.0, 646.0]).len, 391.15);
}
#[test]
fn aurora_anfmikshpp() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([630.0, 697.0], [766.0, 432.0]).len, 391.079);
}
#[test]
fn aurora_lpfwlifyko() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([454.0, 257.0], [816.0, 364.0]).len, 394.071);
}
#[test]
fn aurora_jbmskgkjkj() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([704.0, 369.0], [478.0, 513.0]).len, 398.299);
}
#[test]
fn aurora_nmeaaztpra() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([864.0, 356.0], [662.0, 154.0]).len, 391.296);
}
#[test]
fn aurora_zbgccenckn() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([630.0, 68.0], [544.0, 347.0]).len, 391.383);
}
#[test]
fn aurora_dtrnwyomzs() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([446.0, 318.0], [265.0, 206.0]).len, 399.45);
}
#[test]
fn aurora_dhqnbpulsl() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([136.0, 625.0], [465.0, 460.0]).len, 390.413);
}
#[test]
fn aurora_gdzfuwsyws() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([812.0, 331.0], [600.0, 162.0]).len, 394.792);
}
#[test]
fn aurora_nzgalwluju() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([847.0, 683.0], [521.0, 467.0]).len, 395.976);
}
#[test]
fn aurora_kgvulcfthd() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([380.0, 437.0], [26.0, 600.0]).len, 390.548);
}
#[test]
fn aurora_hubwzradnt() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([380.0, 89.0], [385.0, 351.0]).len, 400.243);
}
#[test]
fn aurora_gjmzjgjvie() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([786.0, 324.0], [411.0, 291.0]).len, 399.437);
}
#[test]
fn aurora_tdpjinqugo() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([642.0, 569.0], [349.0, 366.0]).len, 391.431);
}
#[test]
fn aurora_gvuinpwnbq() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([443.0, 390.0], [192.0, 505.0]).len, 398.109);
}
#[test]
fn aurora_imbaegwoaq() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([467.0, 669.0], [70.0, 661.0]).len, 402.888);
}
#[test]
fn aurora_cyskxmgyjb() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([767.0, 559.0], [786.0, 334.0]).len, 396.009);
}
#[test]
fn aurora_fdivchkdgo() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([529.0, 440.0], [431.0, 456.0]).len, 403.223);
}
#[test]
fn aurora_ampdwaiuig() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([612.0, 47.0], [888.0, 251.0]).len, 398.249);
}
#[test]
fn aurora_omtbksuynu() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([532.0, 381.0], [624.0, 211.0]).len, 398.016);
}
#[test]
fn aurora_gyuoicxaet() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([829.0, 375.0], [454.0, 342.0]).len, 401.96);
}
#[test]
fn aurora_qzqdhkiheo() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([463.0, 61.0], [355.0, 303.0]).len, 399.139);
}
#[test]
fn aurora_yjonxwybro() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([610.0, 319.0], [400.0, 244.0]).len, 403.331);
}
#[test]
fn aurora_zycoscprcl() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([485.0, 373.0], [111.0, 285.0]).len, 397.104);
}
#[test]
fn aurora_hhspaubhiu() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([802.0, 723.0], [439.0, 616.0]).len, 400.644);
}
#[test]
fn aurora_faqxdaflal() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([844.0, 559.0], [784.0, 307.0]).len, 398.74);
}
#[test]
fn aurora_pcjcxmecfk() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([741.0, 397.0], [385.0, 543.0]).len, 402.343);
}
#[test]
fn aurora_ubdscnlfxc() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([509.0, 50.0], [418.0, 306.0]).len, 401.729);
}
#[test]
fn aurora_heakssenpv() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([181.0, 472.0], [504.0, 368.0]).len, 398.876);
}
#[test]
fn aurora_hdyjwwgdtd() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([384.0, 679.0], [627.0, 697.0]).len, 402.887);
}
#[test]
fn aurora_itgvcgxigs() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([610.0, 547.0], [996.0, 455.0]).len, 402.25);
}
#[test]
fn aurora_tzebzrhntn() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([198.0, 570.0], [129.0, 320.0]).len, 406.694);
}
#[test]
fn aurora_cvzufimvtd() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([995.0, 618.0], [610.0, 637.0]).len, 402.078);
}
#[test]
fn aurora_fmsejsaoxs() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([459.0, 479.0], [525.0, 262.0]).len, 403.232);
}
#[test]
fn aurora_ypipcilmos() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([443.0, 194.0], [782.0, 368.0]).len, 399.551);
}
#[test]
fn aurora_lpuyyvbarc() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([560.0, 320.0], [292.0, 118.0]).len, 403.301);
}
#[test]
fn aurora_wfbgjcordi() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([496.0, 543.0], [759.0, 360.0]).len, 409.36);
}
#[test]
fn aurora_bwclgenumb() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([478.0, 328.0], [403.0, 530.0]).len, 402.696);
}
#[test]
fn aurora_bxnbbihutn() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([64.0, 604.0], [49.0, 320.0]).len, 403.551);
}
#[test]
fn aurora_nolslwlxwk() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([609.0, 478.0], [418.0, 314.0]).len, 402.082);
}
#[test]
fn aurora_lkddqjgqml() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([594.0, 557.0], [219.0, 507.0]).len, 410.619);
}
#[test]
fn aurora_ovatexhmep() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([684.0, 102.0], [583.0, 323.0]).len, 403.567);
}
#[test]
fn aurora_qqftsscwze() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([516.0, 565.0], [232.0, 378.0]).len, 413.029);
}
#[test]
fn aurora_ocncinagql() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([464.0, 207.0], [850.0, 102.0]).len, 411.882);
}
#[test]
fn aurora_ojkmczgidg() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([246.0, 607.0], [16.0, 362.0]).len, 410.466);
}
#[test]
fn aurora_lbeebrpzqf() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([81.0, 621.0], [43.0, 315.0]).len, 408.736);
}
#[test]
fn aurora_hxupudqtgu() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([399.0, 716.0], [306.0, 545.0]).len, 403.656);
}
#[test]
fn aurora_zttqkukbqi() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([79.0, 125.0], [111.0, 430.0]).len, 409.152);
}
#[test]
fn aurora_mgbtizkhet() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([757.0, 230.0], [465.0, 289.0]).len, 409.409);
}
#[test]
fn aurora_ytfwgloxxj() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([765.0, 176.0], [511.0, 305.0]).len, 419.386);
}
#[test]
fn aurora_tkurwqijbh() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([693.0, 113.0], [309.0, 228.0]).len, 409.659);
}
#[test]
fn aurora_wcixcnrnlw() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([280.0, 77.0], [250.0, 384.0]).len, 411.317);
}
#[test]
fn aurora_aqxyqwefst() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([536.0, 408.0], [285.0, 266.0]).len, 407.221);
}
#[test]
fn aurora_urgqnqpyif() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([737.0, 147.0], [559.0, 310.0]).len, 415.318);
}
#[test]
fn aurora_lpblmeaorp() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([305.0, 139.0], [473.0, 350.0]).len, 411.773);
}
#[test]
fn aurora_gohydehuwm() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([63.0, 154.0], [213.0, 412.0]).len, 409.67);
}
#[test]
fn aurora_yfinwhitlb() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([53.0, 622.0], [9.0, 335.0]).len, 413.208);
}
#[test]
fn aurora_vjpggjelox() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([32.0, 308.0], [328.0, 564.0]).len, 418.83);
}
#[test]
fn aurora_krsjzjavdo() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([431.0, 192.0], [201.0, 412.0]).len, 415.771);
}
#[test]
fn aurora_bddpefbuar() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([87.0, 169.0], [2.0, 421.0]).len, 415.887);
}
#[test]
fn aurora_jwvurnjhiy() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([934.0, 663.0], [526.0, 608.0]).len, 420.271);
}
#[test]
fn aurora_ozcvijasot() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([884.0, 409.0], [589.0, 522.0]).len, 412.703);
}
#[test]
fn aurora_gyvorqjkng() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([154.0, 201.0], [433.0, 392.0]).len, 414.176);
}
#[test]
fn aurora_ixrsqxeape() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([613.0, 303.0], [216.0, 357.0]).len, 411.753);
}
#[test]
fn aurora_fbbpmupapo() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([785.0, 371.0], [523.0, 539.0]).len, 417.059);
}
#[test]
fn aurora_odypxsgnbo() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([130.0, 273.0], [320.0, 567.0]).len, 420.097);
}
#[test]
fn aurora_foeywaywms() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([271.0, 694.0], [632.0, 600.0]).len, 420.676);
}
#[test]
fn aurora_chnzuguwer() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([162.0, 536.0], [168.0, 247.0]).len, 412.399);
}
#[test]
fn aurora_vibaijztdz() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([621.0, 157.0], [620.0, 400.0]).len, 417.794);
}
#[test]
fn aurora_kbnqfaxuot() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([577.0, 371.0], [782.0, 225.0]).len, 416.74);
}
#[test]
fn aurora_dkkkwwzexw() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([142.0, 686.0], [79.0, 366.0]).len, 417.615);
}
#[test]
fn aurora_yxbyhzgwdr() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([399.0, 353.0], [47.0, 480.0]).len, 414.584);
}
#[test]
fn aurora_curjhpwmij() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([83.0, 634.0], [110.0, 325.0]).len, 417.201);
}
#[test]
fn aurora_nbyndgmcoh() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([290.0, 369.0], [569.0, 172.0]).len, 412.043);
}
#[test]
fn aurora_ylwggpyuot() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([265.0, 412.0], [399.0, 242.0]).len, 417.273);
}
#[test]
fn aurora_tcqshywigp() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([693.0, 273.0], [862.0, 552.0]).len, 423.417);
}
#[test]
fn aurora_yqdfnrmdap() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([581.0, 438.0], [819.0, 505.0]).len, 413.794);
}
#[test]
fn aurora_tgfiwoossk() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([829.0, 651.0], [936.0, 335.0]).len, 424.047);
}
#[test]
fn aurora_rmmvdsfuag() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([466.0, 505.0], [634.0, 429.0]).len, 421.057);
}
#[test]
fn aurora_vkzjyoxale() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([576.0, 191.0], [706.0, 375.0]).len, 418.507);
}
#[test]
fn aurora_phsdatwrnt() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([612.0, 270.0], [206.0, 336.0]).len, 421.58);
}
#[test]
fn aurora_dadaivqndt() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([567.0, 558.0], [237.0, 614.0]).len, 426.173);
}
#[test]
fn aurora_smbhhwwztt() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([328.0, 89.0], [234.0, 396.0]).len, 424.918);
}
#[test]
fn aurora_zcbmjhrbgm() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([224.0, 285.0], [349.0, 539.0]).len, 424.938);
}
#[test]
fn aurora_vdlffidyze() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([456.0, 635.0], [880.0, 657.0]).len, 431.499);
}
#[test]
fn aurora_uaeqnwabup() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([721.0, 345.0], [432.0, 418.0]).len, 417.287);
}
#[test]
fn aurora_befqddvtgt() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([450.0, 382.0], [107.0, 491.0]).len, 428.436);
}
#[test]
fn aurora_dnvtmkwnpd() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([523.0, 233.0], [853.0, 102.0]).len, 423.712);
}
#[test]
fn aurora_qbiodxsnzc() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([513.0, 401.0], [335.0, 283.0]).len, 423.937);
}
#[test]
fn aurora_qultqkgbil() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([690.0, 102.0], [281.0, 117.0]).len, 424.367);
}
#[test]
fn aurora_lnbxxcbcxf() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([386.0, 288.0], [502.0, 630.0]).len, 434.657);
}
#[test]
fn aurora_uemuxgjsyb() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([339.0, 360.0], [616.0, 568.0]).len, 422.809);
}
#[test]
fn aurora_lirtpsqplx() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([846.0, 659.0], [489.0, 434.0]).len, 425.043);
}
#[test]
fn aurora_eelyggchxs() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([862.0, 388.0], [823.0, 95.0]).len, 421.173);
}
#[test]
fn aurora_wujignhdpm() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([539.0, 335.0], [123.0, 285.0]).len, 429.989);
}
#[test]
fn aurora_qtlwgjbhoi() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([673.0, 696.0], [334.0, 551.0]).len, 422.286);
}
#[test]
fn aurora_pmxikjmjtv() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([826.0, 425.0], [633.0, 565.0]).len, 423.725);
}
#[test]
fn aurora_evkrzbzytr() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([906.0, 584.0], [489.0, 536.0]).len, 429.246);
}
#[test]
fn aurora_ehgcjvxxfb() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([495.0, 623.0], [722.0, 363.0]).len, 427.88);
}
#[test]
fn aurora_aqsctbwtfe() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([876.0, 585.0], [465.0, 485.0]).len, 427.27);
}
#[test]
fn aurora_rrngdpichi() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([261.0, 413.0], [650.0, 445.0]).len, 424.059);
}
#[test]
fn aurora_padkuhwfgr() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([155.0, 652.0], [580.0, 695.0]).len, 434.844);
}
#[test]
fn aurora_yxgipnybpp() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([484.0, 321.0], [719.0, 249.0]).len, 427.143);
}
#[test]
fn aurora_ymssbovfpc() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([288.0, 589.0], [581.0, 526.0]).len, 427.611);
}
#[test]
fn aurora_rqgvwvgmjt() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([142.0, 166.0], [384.0, 301.0]).len, 423.228);
}
#[test]
fn aurora_tfxzvmnkhu() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([447.0, 149.0], [586.0, 464.0]).len, 429.709);
}
#[test]
fn aurora_xotjvqosnd() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([847.0, 160.0], [461.0, 73.0]).len, 423.783);
}
#[test]
fn aurora_yjfaiswknq() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([757.0, 410.0], [508.0, 168.0]).len, 425.9);
}
#[test]
fn aurora_rgjoahlljx() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([729.0, 82.0], [662.0, 264.0]).len, 426.236);
}
#[test]
fn aurora_ehoqevmbbf() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([726.0, 232.0], [326.0, 91.0]).len, 428.791);
}
#[test]
fn aurora_vnenugnsfq() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([108.0, 291.0], [163.0, 616.0]).len, 432.743);
}
#[test]
fn aurora_mamuvvlzky() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([570.0, 401.0], [754.0, 517.0]).len, 428.601);
}
#[test]
fn aurora_uzwpylmwbe() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([313.0, 622.0], [352.0, 354.0]).len, 433.174);
}
#[test]
fn aurora_ltdqvnyivg() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([549.0, 603.0], [960.0, 573.0]).len, 434.068);
}
#[test]
fn aurora_tmyssubfjc() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([587.0, 334.0], [1002.0, 363.0]).len, 434.586);
}
#[test]
fn aurora_xscnoizoym() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([471.0, 499.0], [127.0, 277.0]).len, 437.855);
}
#[test]
fn aurora_wmcenwqjqs() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([190.0, 509.0], [498.0, 360.0]).len, 431.204);
}
#[test]
fn aurora_kvjbdpbyvo() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([731.0, 276.0], [414.0, 281.0]).len, 429.956);
}
#[test]
fn aurora_gwyrpaatue() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([634.0, 341.0], [297.0, 171.0]).len, 441.632);
}
#[test]
fn aurora_uwjjbcjngp() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([408.0, 704.0], [68.0, 684.0]).len, 440.138);
}
#[test]
fn aurora_fywunxsrcq() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([307.0, 321.0], [457.0, 63.0]).len, 438.653);
}
#[test]
fn aurora_imkweurdlv() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([745.0, 279.0], [590.0, 274.0]).len, 434.65);
}
#[test]
fn aurora_kpbizwscoc() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([322.0, 499.0], [510.0, 344.0]).len, 439.298);
}
#[test]
fn aurora_ryjdajvpxr() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([382.0, 457.0], [609.0, 685.0]).len, 437.802);
}
#[test]
fn aurora_xwwmkadldz() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([249.0, 224.0], [234.0, 416.0]).len, 433.503);
}
#[test]
fn aurora_mmauxwtwjy() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([106.0, 363.0], [138.0, 690.0]).len, 439.785);
}
#[test]
fn aurora_execfcdpda() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([225.0, 50.0], [639.0, 144.0]).len, 434.946);
}
#[test]
fn aurora_nytwpfthyk() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([455.0, 561.0], [734.0, 676.0]).len, 439.071);
}
#[test]
fn aurora_udhqpuezeb() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([743.0, 423.0], [618.0, 724.0]).len, 440.014);
}
#[test]
fn aurora_uitfsgirza() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([251.0, 216.0], [537.0, 357.0]).len, 447.117);
}
#[test]
fn aurora_ngjpewhmqz() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([593.0, 331.0], [918.0, 552.0]).len, 434.761);
}
#[test]
fn aurora_xwaggirdxt() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([271.0, 43.0], [684.0, 179.0]).len, 440.584);
}
#[test]
fn aurora_fzlvmftgcr() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([370.0, 331.0], [656.0, 456.0]).len, 434.808);
}
#[test]
fn aurora_pipzmdplyw() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([64.0, 673.0], [398.0, 700.0]).len, 446.771);
}
#[test]
fn aurora_jddxskpmsx() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([113.0, 389.0], [539.0, 322.0]).len, 440.949);
}
#[test]
fn aurora_zuelqwstsl() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([456.0, 307.0], [780.0, 204.0]).len, 446.48);
}
#[test]
fn aurora_wzlhkixsuc() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([34.0, 312.0], [445.0, 200.0]).len, 442.506);
}
#[test]
fn aurora_lxffhbwhql() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([344.0, 317.0], [683.0, 207.0]).len, 436.972);
}
#[test]
fn aurora_gdsabvysir() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([584.0, 156.0], [147.0, 195.0]).len, 456.86);
}
#[test]
fn aurora_cmygkypemm() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([650.0, 97.0], [511.0, 427.0]).len, 440.539);
}
#[test]
fn aurora_iotfwlkyfk() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([225.0, 124.0], [534.0, 293.0]).len, 442.192);
}
#[test]
fn aurora_ogimayqhdi() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([767.0, 656.0], [922.0, 356.0]).len, 447.14);
}
#[test]
fn aurora_towtiiwfkz() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([17.0, 510.0], [393.0, 696.0]).len, 440.979);
}
#[test]
fn aurora_jpogwazgem() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([410.0, 197.0], [850.0, 134.0]).len, 455.242);
}
#[test]
fn aurora_aqgheystkt() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([461.0, 398.0], [365.0, 79.0]).len, 441.681);
}
#[test]
fn aurora_eonmoyovbr() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([588.0, 579.0], [827.0, 409.0]).len, 444.749);
}
#[test]
fn aurora_antjlwqzwh() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([372.0, 300.0], [73.0, 177.0]).len, 437.185);
}
#[test]
fn aurora_sfuvwzirme() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([671.0, 273.0], [279.0, 85.0]).len, 442.336);
}
#[test]
fn aurora_viisielfwq() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([324.0, 748.0], [666.0, 708.0]).len, 446.15);
}
#[test]
fn aurora_amkxdyxorh() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([853.0, 299.0], [557.0, 517.0]).len, 447.985);
}
#[test]
fn aurora_kitjyvbfez() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([101.0, 442.0], [381.0, 547.0]).len, 444.246);
}
#[test]
fn aurora_gpxhmsncxs() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([563.0, 550.0], [124.0, 509.0]).len, 450.851);
}
#[test]
fn aurora_gyrpjvjuri() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([74.0, 560.0], [137.0, 240.0]).len, 448.51);
}
#[test]
fn aurora_atgjxwnjqm() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([94.0, 424.0], [502.0, 514.0]).len, 445.108);
}
#[test]
fn aurora_bgprzlbtii() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([641.0, 559.0], [246.0, 664.0]).len, 456.713);
}
#[test]
fn aurora_blixfivbin() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([132.0, 435.0], [540.0, 276.0]).len, 446.555);
}
#[test]
fn aurora_ayyfbwljcq() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([693.0, 216.0], [515.0, 387.0]).len, 446.361);
}
#[test]
fn aurora_okuxndcgrb() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([892.0, 442.0], [464.0, 478.0]).len, 445.846);
}
#[test]
fn aurora_lhzuoehspo() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([148.0, 716.0], [101.0, 370.0]).len, 458.984);
}
#[test]
fn aurora_mkaaegmxui() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([433.0, 405.0], [559.0, 142.0]).len, 448.283);
}
#[test]
fn aurora_rcsesivqus() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([252.0, 632.0], [131.0, 293.0]).len, 458.671);
}
#[test]
fn aurora_ejffaaedrn() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([980.0, 369.0], [593.0, 546.0]).len, 455.998);
}
#[test]
fn aurora_mklmmpqhnx() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([566.0, 404.0], [941.0, 459.0]).len, 447.871);
}
#[test]
fn aurora_czkdfeolpt() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([989.0, 356.0], [653.0, 612.0]).len, 454.895);
}
#[test]
fn aurora_mpflyluguk() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([1018.0, 612.0], [664.0, 367.0]).len, 449.968);
}
#[test]
fn aurora_jggbdawcsm() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([641.0, 411.0], [421.0, 538.0]).len, 452.05);
}
#[test]
fn aurora_nsdywjrlvx() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([696.0, 315.0], [308.0, 359.0]).len, 456.803);
}
#[test]
fn aurora_rdjwdyrekh() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([489.0, 329.0], [644.0, 516.0]).len, 454.117);
}
#[test]
fn aurora_aswtrkpjad() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([527.0, 329.0], [785.0, 278.0]).len, 459.407);
}
#[test]
fn aurora_nabkeurzqc() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([908.0, 300.0], [690.0, 594.0]).len, 457.837);
}
#[test]
fn aurora_lqrfnkkrkj() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([320.0, 143.0], [764.0, 200.0]).len, 457.929);
}
#[test]
fn aurora_xrjkhqiqog() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([190.0, 225.0], [558.0, 252.0]).len, 464.633);
}
#[test]
fn aurora_lxpinzsbsf() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([132.0, 300.0], [377.0, 611.0]).len, 464.107);
}
#[test]
fn aurora_pdfdupwcic() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([574.0, 434.0], [354.0, 210.0]).len, 459.375);
}
#[test]
fn aurora_wnryglwpgi() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([366.0, 71.0], [746.0, 312.0]).len, 453.473);
}
#[test]
fn aurora_hmwccphqek() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([363.0, 538.0], [332.0, 619.0]).len, 450.005);
}
#[test]
fn aurora_smgoisvzen() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([115.0, 386.0], [528.0, 462.0]).len, 453.153);
}
#[test]
fn aurora_snjwtmyuxh() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([318.0, 50.0], [608.0, 318.0]).len, 453.569);
}
#[test]
fn aurora_thqpclmryc() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([585.0, 295.0], [745.0, 92.0]).len, 451.598);
}
#[test]
fn aurora_clngokmcce() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([730.0, 152.0], [275.0, 218.0]).len, 469.235);
}
#[test]
fn aurora_ogexjbpexa() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([549.0, 318.0], [518.0, 523.0]).len, 459.476);
}
#[test]
fn aurora_xwwbqkkzxp() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([793.0, 335.0], [911.0, 613.0]).len, 460.876);
}
#[test]
fn aurora_tdqsvhrgac() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([296.0, 396.0], [273.0, 84.0]).len, 458.677);
}
#[test]
fn aurora_vmnnqzqftn() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([316.0, 234.0], [773.0, 165.0]).len, 476.46);
}
#[test]
fn aurora_mlhlbvokea() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([179.0, 169.0], [434.0, 263.0]).len, 458.648);
}
#[test]
fn aurora_dutwdakzst() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([545.0, 266.0], [241.0, 473.0]).len, 453.32);
}
#[test]
fn aurora_kvhbktcble() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([339.0, 337.0], [689.0, 147.0]).len, 457.375);
}
#[test]
fn aurora_ietwlelspu() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([264.0, 536.0], [699.0, 431.0]).len, 457.561);
}
#[test]
fn aurora_jveoeqvgea() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([384.0, 386.0], [272.0, 82.0]).len, 461.096);
}
#[test]
fn aurora_pkujrrbfum() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([633.0, 503.0], [303.0, 665.0]).len, 465.27);
}
#[test]
fn aurora_lbhaaggicf() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([671.0, 695.0], [656.0, 377.0]).len, 469.023);
}
#[test]
fn aurora_erijiuddem() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([784.0, 111.0], [678.0, 323.0]).len, 461.64);
}
#[test]
fn aurora_mcfaacapxi() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([141.0, 306.0], [383.0, 538.0]).len, 467.381);
}
#[test]
fn aurora_nfkcdfhkec() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([504.0, 630.0], [622.0, 432.0]).len, 461.923);
}
#[test]
fn aurora_vgyedzwigv() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([243.0, 668.0], [312.0, 320.0]).len, 471.454);
}
#[test]
fn aurora_qlhvemqexy() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([799.0, 97.0], [703.0, 370.0]).len, 463.419);
}
#[test]
fn aurora_uyqywkwidl() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([72.0, 335.0], [388.0, 97.0]).len, 461.089);
}
#[test]
fn aurora_gutxwcmtyb() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([400.0, 221.0], [458.0, 418.0]).len, 459.115);
}
#[test]
fn aurora_tevkcsgvsh() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([451.0, 512.0], [905.0, 473.0]).len, 467.006);
}
#[test]
fn aurora_cpjqoxpxtl() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([215.0, 460.0], [364.0, 204.0]).len, 465.092);
}
#[test]
fn aurora_snwmwquamq() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([660.0, 623.0], [654.0, 349.0]).len, 463.335);
}
#[test]
fn aurora_flyfycnfda() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([9.0, 371.0], [417.0, 186.0]).len, 459.064);
}
#[test]
fn aurora_dpudeaatmo() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([227.0, 604.0], [389.0, 363.0]).len, 467.833);
}
#[test]
fn aurora_axhphjecdg() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([283.0, 671.0], [533.0, 496.0]).len, 471.053);
}
#[test]
fn aurora_jqwohxaybj() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([477.0, 644.0], [902.0, 484.0]).len, 466.233);
}
#[test]
fn aurora_fhnoxirchy() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([541.0, 379.0], [240.0, 484.0]).len, 464.909);
}
#[test]
fn aurora_nhhhvusobt() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([213.0, 459.0], [341.0, 187.0]).len, 465.939);
}
#[test]
fn aurora_remupxeqwd() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([823.0, 675.0], [375.0, 713.0]).len, 463.853);
}
#[test]
fn aurora_nbrddcdppn() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([981.0, 415.0], [670.0, 652.0]).len, 470.458);
}
#[test]
fn aurora_mvlehvusuh() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([689.0, 536.0], [949.0, 275.0]).len, 465.79);
}
#[test]
fn aurora_hfxltvlgdm() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([756.0, 235.0], [516.0, 357.0]).len, 469.38);
}
#[test]
fn aurora_bkdnfdnuwt() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([23.0, 544.0], [194.0, 254.0]).len, 470.662);
}
#[test]
fn aurora_hnphhcepzp() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([739.0, 574.0], [818.0, 282.0]).len, 474.274);
}
#[test]
fn aurora_jahatpzbir() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([653.0, 113.0], [215.0, 57.0]).len, 465.554);
}
#[test]
fn aurora_tnmyauqrgn() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([639.0, 147.0], [639.0, 427.0]).len, 466.703);
}
#[test]
fn aurora_axufswntyf() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([478.0, 52.0], [557.0, 403.0]).len, 473.214);
}
#[test]
fn aurora_pxporhjgwz() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([808.0, 194.0], [683.0, 452.0]).len, 471.055);
}
#[test]
fn aurora_kqnlwcptyy() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([264.0, 239.0], [671.0, 297.0]).len, 471.527);
}
#[test]
fn aurora_xffzxuhloa() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([454.0, 477.0], [594.0, 223.0]).len, 475.617);
}
#[test]
fn aurora_iiqiajnrme() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([589.0, 319.0], [508.0, 595.0]).len, 472.897);
}
#[test]
fn aurora_gbiufwtdeg() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([948.0, 393.0], [571.0, 420.0]).len, 472.307);
}
#[test]
fn aurora_etzosujwri() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([838.0, 94.0], [507.0, 283.0]).len, 472.239);
}
#[test]
fn aurora_aaestycumg() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([729.0, 278.0], [310.0, 81.0]).len, 470.131);
}
#[test]
fn aurora_jjgjuhxmqf() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([741.0, 56.0], [297.0, 33.0]).len, 469.718);
}
#[test]
fn aurora_bdbcbpetsa() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([431.0, 634.0], [34.0, 384.0]).len, 478.876);
}
#[test]
fn aurora_gbsxotnrou() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([432.0, 427.0], [635.0, 708.0]).len, 476.123);
}
#[test]
fn aurora_woptmgkrsq() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([793.0, 515.0], [320.0, 572.0]).len, 487.151);
}
#[test]
fn aurora_dfcbyndfpu() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([376.0, 529.0], [309.0, 682.0]).len, 470.566);
}
#[test]
fn aurora_urjvvbzwix() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([414.0, 609.0], [61.0, 437.0]).len, 476.724);
}
#[test]
fn aurora_ziughywstv() {
    let aurora = aurora_mesh();
    known_failure!(aurora.path([960.0, 650.0], [623.0, 410.0]).len, 482.714);
}
#[test]
fn aurora_lozazicsuj() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([147.0, 346.0], [374.0, 651.0]).len, 481.703);
}
#[test]
fn aurora_chfpenyzte() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([415.0, 435.0], [681.0, 422.0]).len, 471.401);
}
#[test]
fn aurora_ivhmybtqrs() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([740.0, 419.0], [290.0, 528.0]).len, 472.62);
}
#[test]
fn aurora_cwaenqxdqg() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([545.0, 23.0], [99.0, 150.0]).len, 473.543);
}
#[test]
fn aurora_wbevqknuma() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([234.0, 361.0], [528.0, 622.0]).len, 484.101);
}
#[test]
fn aurora_jcehzrwkmf() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([152.0, 590.0], [600.0, 619.0]).len, 480.639);
}
#[test]
fn aurora_pxsfmlmtbl() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([96.0, 117.0], [542.0, 212.0]).len, 483.633);
}
#[test]
fn aurora_vurfcpioxb() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([416.0, 419.0], [769.0, 362.0]).len, 477.74);
}
#[test]
fn aurora_yxbtzaduap() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([953.0, 562.0], [524.0, 708.0]).len, 479.178);
}
#[test]
fn aurora_spcslvkcxe() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([30.0, 477.0], [497.0, 516.0]).len, 485.753);
}
#[test]
fn aurora_encpyjihts() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([603.0, 549.0], [481.0, 332.0]).len, 483.533);
}
#[test]
fn aurora_jlhjlzaedg() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([33.0, 583.0], [348.0, 383.0]).len, 476.255);
}
#[test]
fn aurora_bubklelemd() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([179.0, 173.0], [655.0, 113.0]).len, 490.491);
}
#[test]
fn aurora_szuqvacgwp() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([932.0, 156.0], [682.0, 450.0]).len, 485.575);
}
#[test]
fn aurora_ymrznubiha() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([485.0, 446.0], [904.0, 665.0]).len, 482.32);
}
#[test]
fn aurora_oixrptrilg() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([147.0, 105.0], [32.0, 430.0]).len, 480.632);
}
#[test]
fn aurora_dtovkepjpp() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([376.0, 575.0], [806.0, 717.0]).len, 477.084);
}
#[test]
fn aurora_tqmggsekph() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([482.0, 581.0], [55.0, 671.0]).len, 483.528);
}
#[test]
fn aurora_ghcupqynbi() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([657.0, 575.0], [490.0, 374.0]).len, 487.415);
}
#[test]
fn aurora_kbmfzsbatn() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([1011.0, 311.0], [926.0, 649.0]).len, 488.482);
}
#[test]
fn aurora_ptairquxtc() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([502.0, 388.0], [731.0, 242.0]).len, 481.328);
}
#[test]
fn aurora_nvhunhkriw() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([474.0, 558.0], [38.0, 400.0]).len, 482.414);
}
#[test]
fn aurora_rkewbwlyjn() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([609.0, 653.0], [974.0, 396.0]).len, 480.578);
}
#[test]
fn aurora_vreohvfpnq() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([759.0, 552.0], [315.0, 370.0]).len, 482.398);
}
#[test]
fn aurora_gikgeeufvu() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([806.0, 92.0], [935.0, 413.0]).len, 490.011);
}
#[test]
fn aurora_qsjcmfaszt() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([548.0, 428.0], [983.0, 361.0]).len, 492.026);
}
#[test]
fn aurora_febjanries() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([696.0, 705.0], [282.0, 668.0]).len, 485.222);
}
#[test]
fn aurora_yfsfsmvydo() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([745.0, 251.0], [708.0, 401.0]).len, 484.553);
}
#[test]
fn aurora_chjqoeuhwo() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([456.0, 242.0], [79.0, 469.0]).len, 494.778);
}
#[test]
fn aurora_nwjbxemtsd() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([381.0, 224.0], [794.0, 207.0]).len, 492.921);
}
#[test]
fn aurora_wvdglvxfwk() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([63.0, 334.0], [167.0, 734.0]).len, 492.445);
}
#[test]
fn aurora_qyewdriuxv() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([570.0, 698.0], [333.0, 468.0]).len, 492.232);
}
#[test]
fn aurora_doikuzwdmz() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([411.0, 583.0], [877.0, 688.0]).len, 485.689);
}
#[test]
fn aurora_iwdevautpq() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([489.0, 600.0], [478.0, 318.0]).len, 491.786);
}
#[test]
fn aurora_rczmzaetnh() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([243.0, 423.0], [577.0, 204.0]).len, 491.426);
}
#[test]
fn aurora_ebvvnbrsgb() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([609.0, 644.0], [883.0, 363.0]).len, 487.707);
}
#[test]
fn aurora_doxednzgfo() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([925.0, 594.0], [463.0, 447.0]).len, 487.106);
}
#[test]
fn aurora_ssxkntnsxs() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([203.0, 181.0], [681.0, 85.0]).len, 500.275);
}
#[test]
fn aurora_bsjtvkdlxn() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([693.0, 533.0], [875.0, 267.0]).len, 491.335);
}
#[test]
fn aurora_lkgekvlfxj() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([197.0, 305.0], [43.0, 665.0]).len, 499.197);
}
#[test]
fn aurora_woavgbpsnj() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([441.0, 203.0], [205.0, 442.0]).len, 492.786);
}
#[test]
fn aurora_frfxlzodmz() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([768.0, 166.0], [727.0, 402.0]).len, 488.639);
}
#[test]
fn aurora_ngoneftscq() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([193.0, 179.0], [548.0, 342.0]).len, 497.748);
}
#[test]
fn aurora_pyybxrtfbt() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([429.0, 146.0], [716.0, 422.0]).len, 489.641);
}
#[test]
fn aurora_kuyhilgnal() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([470.0, 687.0], [28.0, 540.0]).len, 485.16);
}
#[test]
fn aurora_ityygmyili() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([485.0, 644.0], [644.0, 347.0]).len, 494.053);
}
#[test]
fn aurora_ehsjdoyfzk() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([83.0, 317.0], [311.0, 641.0]).len, 499.216);
}
#[test]
fn aurora_tzlcrmkmzr() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([37.0, 651.0], [491.0, 486.0]).len, 496.928);
}
#[test]
fn aurora_zkpzpqpola() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([594.0, 723.0], [420.0, 473.0]).len, 492.653);
}
#[test]
fn aurora_gzwejwovbv() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([415.0, 599.0], [913.0, 614.0]).len, 508.97);
}
#[test]
fn aurora_snojvlqume() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([505.0, 355.0], [519.0, 583.0]).len, 500.307);
}
#[test]
fn aurora_lwelfvtwdo() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([999.0, 438.0], [753.0, 122.0]).len, 494.758);
}
#[test]
fn aurora_hfdtbcmoeo() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([127.0, 306.0], [604.0, 262.0]).len, 494.298);
}
#[test]
fn aurora_vtvmvlmbkf() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([585.0, 574.0], [941.0, 433.0]).len, 499.593);
}
#[test]
fn aurora_kocfqpifxn() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([548.0, 430.0], [127.0, 439.0]).len, 495.496);
}
#[test]
fn aurora_rpvkjtoncj() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([707.0, 694.0], [263.0, 741.0]).len, 493.97);
}
#[test]
fn aurora_jmlngbvlky() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([284.0, 72.0], [759.0, 80.0]).len, 493.851);
}
#[test]
fn aurora_nxmzvsnpbc() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([564.0, 571.0], [155.0, 353.0]).len, 511.745);
}
#[test]
fn aurora_bozbprakrd() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([332.0, 221.0], [803.0, 283.0]).len, 500.716);
}
#[test]
fn aurora_nfryvienmd() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([441.0, 472.0], [428.0, 197.0]).len, 505.727);
}
#[test]
fn aurora_gyamrpttii() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([441.0, 80.0], [141.0, 402.0]).len, 499.754);
}
#[test]
fn aurora_fmddoiffcs() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([601.0, 278.0], [870.0, 155.0]).len, 495.758);
}
#[test]
fn aurora_lebmmatlvu() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([353.0, 293.0], [561.0, 463.0]).len, 503.222);
}
#[test]
fn aurora_dlodlueyza() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([157.0, 150.0], [666.0, 174.0]).len, 515.258);
}
#[test]
fn aurora_ptfkabhmil() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([326.0, 516.0], [310.0, 228.0]).len, 502.646);
}
#[test]
fn aurora_zuqzzdjnhn() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([643.0, 672.0], [760.0, 399.0]).len, 502.887);
}
#[test]
fn aurora_quqvrmsetf() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([630.0, 342.0], [239.0, 247.0]).len, 503.551);
}
#[test]
fn aurora_gtzjwlnlvv() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([611.0, 281.0], [199.0, 161.0]).len, 503.642);
}
#[test]
fn aurora_jyrdtjpuyz() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([626.0, 390.0], [754.0, 228.0]).len, 500.934);
}
#[test]
fn aurora_sqzaxdzaio() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([573.0, 165.0], [103.0, 292.0]).len, 511.339);
}
#[test]
fn aurora_pckgvbjxjw() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([120.0, 386.0], [580.0, 427.0]).len, 498.937);
}
#[test]
fn aurora_xdoaxuljxc() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([981.0, 419.0], [547.0, 485.0]).len, 500.841);
}
#[test]
fn aurora_vohccfhoiv() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([660.0, 215.0], [185.0, 76.0]).len, 499.544);
}
#[test]
fn aurora_eufnaikinb() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([269.0, 261.0], [650.0, 360.0]).len, 502.568);
}
#[test]
fn aurora_kfybndkuap() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([236.0, 218.0], [745.0, 172.0]).len, 522.283);
}
#[test]
fn aurora_fvqcifenti() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([392.0, 518.0], [510.0, 193.0]).len, 507.167);
}
#[test]
fn aurora_rpiasbqdrr() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([454.0, 534.0], [581.0, 294.0]).len, 509.588);
}
#[test]
fn aurora_pqmxfsemed() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([126.0, 268.0], [122.0, 645.0]).len, 510.925);
}
#[test]
fn aurora_xoamhykqfx() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([280.0, 623.0], [122.0, 245.0]).len, 512.137);
}
#[test]
fn aurora_tsuqddgjox() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([167.0, 708.0], [35.0, 309.0]).len, 505.972);
}
#[test]
fn aurora_ufhiposogv() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([108.0, 156.0], [420.0, 273.0]).len, 500.906);
}
#[test]
fn aurora_zsybcpihge() {
    let aurora = aurora_mesh();
    known_failure!(aurora.path([764.0, 507.0], [462.0, 296.0]).len, 504.778);
}
#[test]
fn aurora_dvsgonyhvn() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([861.0, 388.0], [600.0, 104.0]).len, 505.278);
}
#[test]
fn aurora_cfjaawlzcl() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([497.0, 549.0], [555.0, 430.0]).len, 509.249);
}
#[test]
fn aurora_dmmlzgfrfb() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([324.0, 654.0], [272.0, 349.0]).len, 511.787);
}
#[test]
fn aurora_gowhadidnu() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([505.0, 317.0], [227.0, 560.0]).len, 507.114);
}
#[test]
fn aurora_mdfiscjuvn() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([748.0, 149.0], [887.0, 481.0]).len, 508.815);
}
#[test]
fn aurora_qawexqmjik() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([433.0, 312.0], [70.0, 189.0]).len, 503.959);
}
#[test]
fn aurora_dqjvofkpri() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([594.0, 208.0], [707.0, 426.0]).len, 510.3);
}
#[test]
fn aurora_lbvzhvslmf() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([764.0, 201.0], [546.0, 414.0]).len, 510.129);
}
#[test]
fn aurora_bzlxllphny() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([645.0, 168.0], [228.0, 352.0]).len, 514.682);
}
#[test]
fn aurora_bhzlmfadix() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([220.0, 415.0], [167.0, 121.0]).len, 517.634);
}
#[test]
fn aurora_vazddvfphl() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([353.0, 458.0], [616.0, 443.0]).len, 514.181);
}
#[test]
fn aurora_jwgxoegapc() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([826.0, 292.0], [696.0, 616.0]).len, 515.78);
}
#[test]
fn aurora_joynawzppy() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([520.0, 638.0], [452.0, 267.0]).len, 518.081);
}
#[test]
fn aurora_lerrdkdwqy() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([903.0, 381.0], [614.0, 671.0]).len, 512.421);
}
#[test]
fn aurora_txiwtgywbx() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([569.0, 394.0], [769.0, 275.0]).len, 506.105);
}
#[test]
fn aurora_tlaseckjfa() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([862.0, 269.0], [506.0, 516.0]).len, 514.74);
}
#[test]
fn aurora_bplpjgbjvc() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([286.0, 205.0], [651.0, 369.0]).len, 521.171);
}
#[test]
fn aurora_segmbjvcyf() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([988.0, 586.0], [806.0, 321.0]).len, 510.488);
}
#[test]
fn aurora_gbnugpxqcg() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([742.0, 103.0], [709.0, 397.0]).len, 515.466);
}
#[test]
fn aurora_ojevuparab() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([975.0, 466.0], [486.0, 413.0]).len, 513.562);
}
#[test]
fn aurora_whqpuawwsw() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([798.0, 325.0], [358.0, 365.0]).len, 516.42);
}
#[test]
fn aurora_xhjszvjley() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([506.0, 660.0], [25.0, 534.0]).len, 511.684);
}
#[test]
fn aurora_zxmvvpewad() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([863.0, 431.0], [388.0, 343.0]).len, 510.944);
}
#[test]
fn aurora_dchokuenqv() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([162.0, 476.0], [548.0, 701.0]).len, 510.9);
}
#[test]
fn aurora_tpvktjkokj() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([546.0, 656.0], [867.0, 394.0]).len, 518.093);
}
#[test]
fn aurora_gdsboyyjnl() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([426.0, 133.0], [227.0, 413.0]).len, 511.548);
}
#[test]
fn aurora_jnxfksdxmk() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([371.0, 604.0], [637.0, 405.0]).len, 516.361);
}
#[test]
fn aurora_jpftbwxwuh() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([196.0, 664.0], [658.0, 608.0]).len, 527.336);
}
#[test]
fn aurora_ywozuzzngq() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([305.0, 547.0], [341.0, 273.0]).len, 526.123);
}
#[test]
fn aurora_glqxqdspza() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([106.0, 190.0], [237.0, 474.0]).len, 518.445);
}
#[test]
fn aurora_dicclfvmea() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([151.0, 301.0], [255.0, 712.0]).len, 524.833);
}
#[test]
fn aurora_lglrlehxmb() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([639.0, 409.0], [698.0, 114.0]).len, 516.397);
}
#[test]
fn aurora_futftzymiy() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([336.0, 587.0], [498.0, 394.0]).len, 523.083);
}
#[test]
fn aurora_rfxgfnprup() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([552.0, 331.0], [231.0, 533.0]).len, 523.221);
}
#[test]
fn aurora_kcmsfoboas() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([333.0, 196.0], [847.0, 150.0]).len, 528.238);
}
#[test]
fn aurora_vzqgaxnvbj() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([572.0, 19.0], [895.0, 289.0]).len, 519.295);
}
#[test]
fn aurora_tfemtzxgol() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([454.0, 335.0], [782.0, 78.0]).len, 517.51);
}
#[test]
fn aurora_ywzlxwcysb() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([23.0, 483.0], [359.0, 533.0]).len, 525.71);
}
#[test]
fn aurora_yxxwrehctu() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([807.0, 450.0], [391.0, 639.0]).len, 528.322);
}
#[test]
fn aurora_xmojihqhjx() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([446.0, 380.0], [830.0, 189.0]).len, 524.155);
}
#[test]
fn aurora_poohvkifbh() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([630.0, 604.0], [520.0, 344.0]).len, 522.982);
}
#[test]
fn aurora_tjzbryhxhp() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([705.0, 305.0], [217.0, 149.0]).len, 524.068);
}
#[test]
fn aurora_xibegcygyd() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([665.0, 313.0], [151.0, 262.0]).len, 528.171);
}
#[test]
fn aurora_xcmqimtkvx() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([583.0, 648.0], [968.0, 433.0]).len, 522.613);
}
#[test]
fn aurora_qbrenxzcgp() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([529.0, 355.0], [50.0, 466.0]).len, 520.719);
}
#[test]
fn aurora_zytdbsawle() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([862.0, 413.0], [520.0, 615.0]).len, 522.422);
}
#[test]
fn aurora_saxenrnuqe() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([518.0, 388.0], [38.0, 196.0]).len, 524.49);
}
#[test]
fn aurora_zkijnswnzd() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([783.0, 277.0], [298.0, 99.0]).len, 526.083);
}
#[test]
fn aurora_goybubqfge() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([616.0, 737.0], [369.0, 492.0]).len, 526.081);
}
#[test]
fn aurora_yetugvgjmy() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([324.0, 610.0], [789.0, 694.0]).len, 523.457);
}
#[test]
fn aurora_pfqhxnchhf() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([143.0, 192.0], [250.0, 590.0]).len, 530.199);
}
#[test]
fn aurora_piwpksqhpb() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([415.0, 404.0], [92.0, 177.0]).len, 522.84);
}
#[test]
fn aurora_vnpdgubkwe() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([288.0, 673.0], [653.0, 453.0]).len, 531.232);
}
#[test]
fn aurora_fbmpnqhpaf() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([480.0, 606.0], [517.0, 338.0]).len, 534.49);
}
#[test]
fn aurora_lhjzsexjyo() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([123.0, 310.0], [542.0, 576.0]).len, 536.104);
}
#[test]
fn aurora_vhdmsppssc() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([830.0, 169.0], [299.0, 163.0]).len, 541.875);
}
#[test]
fn aurora_utenjbnfux() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([307.0, 617.0], [404.0, 364.0]).len, 530.291);
}
#[test]
fn aurora_pubsdouypa() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([908.0, 577.0], [384.0, 533.0]).len, 534.947);
}
#[test]
fn aurora_plahwauxma() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([673.0, 346.0], [702.0, 58.0]).len, 528.087);
}
#[test]
fn aurora_wqtlmdwbtc() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([241.0, 262.0], [243.0, 642.0]).len, 532.829);
}
#[test]
fn aurora_rduomsiuid() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([521.0, 469.0], [31.0, 308.0]).len, 534.416);
}
#[test]
fn aurora_bqgtzclymz() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([441.0, 429.0], [822.0, 277.0]).len, 526.544);
}
#[test]
fn aurora_doymvxmcde() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([288.0, 733.0], [759.0, 513.0]).len, 539.269);
}
#[test]
fn aurora_oowgjcaxhy() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([586.0, 461.0], [344.0, 291.0]).len, 528.907);
}
#[test]
fn aurora_qkjfjpxjcf() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([132.0, 247.0], [645.0, 351.0]).len, 534.687);
}
#[test]
fn aurora_tjroaubipo() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([443.0, 75.0], [811.0, 401.0]).len, 531.169);
}
#[test]
fn aurora_dgycmjxqjj() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([502.0, 414.0], [1002.0, 400.0]).len, 529.984);
}
#[test]
fn aurora_ppbhzmksgx() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([863.0, 284.0], [766.0, 690.0]).len, 545.17);
}
#[test]
fn aurora_nnaenbubdt() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([704.0, 421.0], [228.0, 360.0]).len, 527.835);
}
#[test]
fn aurora_caorbrwxhx() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([733.0, 598.0], [224.0, 541.0]).len, 534.313);
}
#[test]
fn aurora_iuxqjufqug() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([770.0, 225.0], [792.0, 542.0]).len, 537.68);
}
#[test]
fn aurora_kzeacpsijl() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([868.0, 361.0], [408.0, 136.0]).len, 537.724);
}
#[test]
fn aurora_aefmphhcrn() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([610.0, 435.0], [660.0, 71.0]).len, 535.392);
}
#[test]
fn aurora_lletjhtwae() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([354.0, 461.0], [401.0, 153.0]).len, 536.449);
}
#[test]
fn aurora_binoxyfhjq() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([514.0, 445.0], [1009.0, 621.0]).len, 528.697);
}
#[test]
fn aurora_rlqbjkjuaf() {
    let aurora = aurora_mesh();
    known_failure!(aurora.path([834.0, 630.0], [550.0, 283.0]).len, 530.87);
}
#[test]
fn aurora_ywyeulyguv() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([245.0, 295.0], [52.0, 634.0]).len, 539.564);
}
#[test]
fn aurora_zzctlmlinw() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([396.0, 100.0], [137.0, 484.0]).len, 539.577);
}
#[test]
fn aurora_jjpdyeeklr() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([418.0, 347.0], [216.0, 658.0]).len, 541.428);
}
#[test]
fn aurora_xnmftwfodc() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([685.0, 359.0], [280.0, 539.0]).len, 536.787);
}
#[test]
fn aurora_onhwrzswcu() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([229.0, 290.0], [555.0, 674.0]).len, 548.793);
}
#[test]
fn aurora_ynzxjjehne() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([421.0, 135.0], [75.0, 488.0]).len, 533.52);
}
#[test]
fn aurora_lndtsaqcgf() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([679.0, 703.0], [212.0, 639.0]).len, 543.561);
}
#[test]
fn aurora_jbsjjbedcb() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([477.0, 203.0], [944.0, 390.0]).len, 532.576);
}
#[test]
fn aurora_urfoehtcqi() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([57.0, 154.0], [592.0, 192.0]).len, 548.47);
}
#[test]
fn aurora_xxmbxdghyb() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([315.0, 635.0], [396.0, 358.0]).len, 536.34);
}
#[test]
fn aurora_ekeipobqoz() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([36.0, 362.0], [369.0, 524.0]).len, 546.326);
}
#[test]
fn aurora_hvkktnzrbq() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([277.0, 507.0], [577.0, 400.0]).len, 540.861);
}
#[test]
fn aurora_sssueclcln() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([907.0, 585.0], [396.0, 680.0]).len, 548.847);
}
#[test]
fn aurora_kamgsfhewt() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([116.0, 400.0], [491.0, 663.0]).len, 535.653);
}
#[test]
fn aurora_kiowtchpuu() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([43.0, 383.0], [421.0, 673.0]).len, 540.353);
}
#[test]
fn aurora_zepxhmnmdn() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([643.0, 294.0], [476.0, 636.0]).len, 540.244);
}
#[test]
fn aurora_sgsuphvpae() {
    let aurora = aurora_mesh();
    known_failure!(aurora.path([916.0, 624.0], [589.0, 269.0]).len, 541.102);
}
#[test]
fn aurora_hvwpzrpqep() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([248.0, 482.0], [376.0, 165.0]).len, 541.162);
}
#[test]
fn aurora_dstexhbhlg() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([426.0, 518.0], [835.0, 378.0]).len, 544.14);
}
#[test]
fn aurora_ouiwlagrgi() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([504.0, 265.0], [733.0, 565.0]).len, 540.685);
}
#[test]
fn aurora_qrvlsiefba() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([17.0, 390.0], [555.0, 326.0]).len, 550.582);
}
#[test]
fn aurora_ghqcyrdxrx() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([11.0, 321.0], [329.0, 690.0]).len, 552.94);
}
#[test]
fn aurora_jqtenfbsla() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([207.0, 712.0], [126.0, 295.0]).len, 556.698);
}
#[test]
fn aurora_iyituhsjum() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([440.0, 685.0], [951.0, 575.0]).len, 550.166);
}
#[test]
fn aurora_tvxivgmgay() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([95.0, 534.0], [348.0, 738.0]).len, 537.337);
}
#[test]
fn aurora_ccatqoukvj() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([950.0, 565.0], [417.0, 471.0]).len, 546.343);
}
#[test]
fn aurora_dvredemlcl() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([786.0, 395.0], [318.0, 376.0]).len, 539.337);
}
#[test]
fn aurora_gvtucuicyf() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([947.0, 285.0], [845.0, 702.0]).len, 544.114);
}
#[test]
fn aurora_xjsfyhjlzg() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([457.0, 618.0], [630.0, 293.0]).len, 545.544);
}
#[test]
fn aurora_bnyogggabg() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([324.0, 554.0], [571.0, 262.0]).len, 546.455);
}
#[test]
fn aurora_keazaipzyd() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([378.0, 370.0], [692.0, 575.0]).len, 546.046);
}
#[test]
fn aurora_ypogkeismt() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([354.0, 360.0], [752.0, 138.0]).len, 547.526);
}
#[test]
fn aurora_zjgljpdczv() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([359.0, 510.0], [16.0, 580.0]).len, 550.431);
}
#[test]
fn aurora_xaillzkaak() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([694.0, 273.0], [1011.0, 600.0]).len, 552.206);
}
#[test]
fn aurora_sornxmxxec() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([255.0, 228.0], [573.0, 427.0]).len, 554.65);
}
#[test]
fn aurora_pmkwdgleyl() {
    let aurora = aurora_mesh();
    known_failure!(aurora.path([933.0, 616.0], [643.0, 254.0]).len, 556.515);
}
#[test]
fn aurora_ruaguvjkdq() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([771.0, 503.0], [270.0, 643.0]).len, 563.121);
}
#[test]
fn aurora_djmdatpwkv() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([16.0, 463.0], [437.0, 161.0]).len, 546.851);
}
#[test]
fn aurora_guztwjoisj() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([929.0, 354.0], [610.0, 667.0]).len, 552.821);
}
#[test]
fn aurora_apvrgwtdvw() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([277.0, 250.0], [780.0, 299.0]).len, 547.361);
}
#[test]
fn aurora_horhandonr() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([816.0, 275.0], [425.0, 543.0]).len, 557.816);
}
#[test]
fn aurora_bbrterupwe() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([346.0, 176.0], [853.0, 315.0]).len, 546.587);
}
#[test]
fn aurora_hxccfqvltf() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([207.0, 315.0], [154.0, 729.0]).len, 557.65);
}
#[test]
fn aurora_cxwjbknifv() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([1010.0, 446.0], [473.0, 469.0]).len, 554.594);
}
#[test]
fn aurora_gbxhpdjonw() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([397.0, 499.0], [60.0, 216.0]).len, 552.321);
}
#[test]
fn aurora_rwexvabqgm() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([178.0, 410.0], [677.0, 403.0]).len, 548.615);
}
#[test]
fn aurora_cnxqxpfyrx() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([905.0, 425.0], [567.0, 123.0]).len, 552.717);
}
#[test]
fn aurora_mehnvbuuqk() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([155.0, 529.0], [35.0, 152.0]).len, 554.842);
}
#[test]
fn aurora_lgfvzzcnsg() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([885.0, 398.0], [481.0, 628.0]).len, 555.473);
}
#[test]
fn aurora_jveitwlcnc() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([6.0, 334.0], [241.0, 730.0]).len, 561.718);
}
#[test]
fn aurora_jtfbriuuen() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([529.0, 37.0], [911.0, 300.0]).len, 551.075);
}
#[test]
fn aurora_wfjytoqvbn() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([23.0, 478.0], [577.0, 477.0]).len, 564.11);
}
#[test]
fn aurora_gwaexsetak() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([471.0, 387.0], [271.0, 609.0]).len, 558.753);
}
#[test]
fn aurora_jgdddldqoe() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([35.0, 665.0], [122.0, 234.0]).len, 564.869);
}
#[test]
fn aurora_ofmjgnpahs() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([727.0, 551.0], [817.0, 197.0]).len, 561.595);
}
#[test]
fn aurora_waptshlgpf() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([858.0, 502.0], [739.0, 263.0]).len, 559.491);
}
#[test]
fn aurora_qqcowxghyn() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([742.0, 701.0], [952.0, 309.0]).len, 564.684);
}
#[test]
fn aurora_jtahcameji() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([262.0, 234.0], [789.0, 290.0]).len, 556.218);
}
#[test]
fn aurora_pfmticgmaq() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([249.0, 42.0], [731.0, 300.0]).len, 553.291);
}
#[test]
fn aurora_mcfiqclhqd() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([768.0, 684.0], [345.0, 472.0]).len, 555.376);
}
#[test]
fn aurora_jsdhufhtzp() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([445.0, 68.0], [11.0, 321.0]).len, 559.391);
}
#[test]
fn aurora_ytrurmyxhn() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([949.0, 273.0], [421.0, 130.0]).len, 558.092);
}
#[test]
fn aurora_amosavobuf() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([890.0, 356.0], [594.0, 711.0]).len, 561.182);
}
#[test]
fn aurora_scvcczktrn() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([574.0, 635.0], [259.0, 253.0]).len, 569.436);
}
#[test]
fn aurora_vlfsblxlat() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([287.0, 121.0], [829.0, 180.0]).len, 563.879);
}
#[test]
fn aurora_dmhgaupzsf() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([721.0, 279.0], [830.0, 517.0]).len, 559.339);
}
#[test]
fn aurora_ycnblspwdz() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([801.0, 636.0], [271.0, 645.0]).len, 566.869);
}
#[test]
fn aurora_sdoktwrrjv() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([678.0, 703.0], [937.0, 295.0]).len, 561.242);
}
#[test]
fn aurora_jzsbojxxvm() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([851.0, 222.0], [912.0, 615.0]).len, 562.728);
}
#[test]
fn aurora_cejgptksix() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([772.0, 195.0], [217.0, 202.0]).len, 574.995);
}
#[test]
fn aurora_lpudtxedrm() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([473.0, 228.0], [934.0, 462.0]).len, 560.844);
}
#[test]
fn aurora_rbyiuhtcen() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([560.0, 645.0], [16.0, 589.0]).len, 571.298);
}
#[test]
fn aurora_ikarfblqed() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([585.0, 671.0], [444.0, 302.0]).len, 567.969);
}
#[test]
fn aurora_bykjlezxkt() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([730.0, 396.0], [330.0, 235.0]).len, 563.182);
}
#[test]
fn aurora_znjqrdrhab() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([94.0, 380.0], [626.0, 389.0]).len, 563.028);
}
#[test]
fn aurora_eeqpdyjufp() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([441.0, 465.0], [893.0, 378.0]).len, 569.485);
}
#[test]
fn aurora_uuogluejdh() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([421.0, 626.0], [303.0, 254.0]).len, 567.494);
}
#[test]
fn aurora_mfldzxblrx() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([361.0, 221.0], [209.0, 533.0]).len, 561.595);
}
#[test]
fn aurora_ekhoonwryp() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([230.0, 212.0], [323.0, 527.0]).len, 565.73);
}
#[test]
fn aurora_lpemdrngms() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([9.0, 476.0], [213.0, 61.0]).len, 567.751);
}
#[test]
fn aurora_vxuywmoknh() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([916.0, 280.0], [407.0, 93.0]).len, 563.881);
}
#[test]
fn aurora_dzdlotinnd() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([475.0, 642.0], [498.0, 395.0]).len, 573.881);
}
#[test]
fn aurora_pprebozafw() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([668.0, 259.0], [868.0, 694.0]).len, 573.343);
}
#[test]
fn aurora_xixvdciqlw() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([551.0, 578.0], [919.0, 264.0]).len, 568.176);
}
#[test]
fn aurora_brkhcjpflk() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([725.0, 586.0], [199.0, 588.0]).len, 567.395);
}
#[test]
fn aurora_avvhhskxzj() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([680.0, 434.0], [355.0, 205.0]).len, 570.267);
}
#[test]
fn aurora_cpijykclck() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([772.0, 353.0], [640.0, 691.0]).len, 571.901);
}
#[test]
fn aurora_txfhmwnzgy() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([584.0, 657.0], [51.0, 503.0]).len, 566.313);
}
#[test]
fn aurora_leojexzuyc() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([39.0, 426.0], [561.0, 265.0]).len, 568.842);
}
#[test]
fn aurora_kyyueaamzh() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([678.0, 72.0], [1013.0, 427.0]).len, 573.34);
}
#[test]
fn aurora_ujeddlutqf() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([17.0, 450.0], [512.0, 418.0]).len, 574.169);
}
#[test]
fn aurora_didssvjtol() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([263.0, 502.0], [799.0, 654.0]).len, 570.491);
}
#[test]
fn aurora_txeuadwgek() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([245.0, 355.0], [793.0, 354.0]).len, 576.308);
}
#[test]
fn aurora_fzksqqhuvc() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([193.0, 232.0], [739.0, 241.0]).len, 574.394);
}
#[test]
fn aurora_cjltqvmeqf() {
    let aurora = aurora_mesh();
    known_failure!(aurora.path([944.0, 573.0], [508.0, 279.0]).len, 566.158);
}
#[test]
fn aurora_hxhrkvrjke() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([21.0, 320.0], [563.0, 389.0]).len, 573.278);
}
#[test]
fn aurora_pdgwzthtgw() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([454.0, 420.0], [653.0, 193.0]).len, 565.703);
}
#[test]
fn aurora_cfnhnvayiw() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([503.0, 246.0], [711.0, 563.0]).len, 570.109);
}
#[test]
fn aurora_gpcsrgamsf() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([582.0, 588.0], [48.0, 561.0]).len, 574.379);
}
#[test]
fn aurora_vfwruwtnmy() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([12.0, 404.0], [435.0, 109.0]).len, 571.821);
}
#[test]
fn aurora_adcszjemvi() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([452.0, 336.0], [168.0, 88.0]).len, 572.912);
}
#[test]
fn aurora_bcgftmcwtr() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([605.0, 643.0], [186.0, 408.0]).len, 571.507);
}
#[test]
fn aurora_nmdsgyxhte() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([86.0, 134.0], [634.0, 50.0]).len, 572.338);
}
#[test]
fn aurora_anepxribyu() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([482.0, 296.0], [751.0, 593.0]).len, 571.192);
}
#[test]
fn aurora_ptvjfuovyh() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([812.0, 279.0], [400.0, 544.0]).len, 579.393);
}
#[test]
fn aurora_mjokehtvxr() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([720.0, 413.0], [311.0, 598.0]).len, 573.382);
}
#[test]
fn aurora_qbmfxqwjmw() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([828.0, 212.0], [299.0, 241.0]).len, 579.497);
}
#[test]
fn aurora_kexfkvpatx() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([117.0, 326.0], [628.0, 226.0]).len, 580.356);
}
#[test]
fn aurora_ehjyadocpy() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([249.0, 203.0], [303.0, 543.0]).len, 577.132);
}
#[test]
fn aurora_qjyknlskrt() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([430.0, 455.0], [887.0, 403.0]).len, 575.539);
}
#[test]
fn aurora_vfbmvnadrt() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([685.0, 416.0], [313.0, 474.0]).len, 572.599);
}
#[test]
fn aurora_wktqbgcbvz() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([512.0, 573.0], [392.0, 272.0]).len, 590.156);
}
#[test]
fn aurora_xsgogfklza() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([145.0, 647.0], [692.0, 578.0]).len, 593.825);
}
#[test]
fn aurora_twgbaxceig() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([933.0, 141.0], [604.0, 489.0]).len, 579.788);
}
#[test]
fn aurora_srwkgeuytd() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([581.0, 642.0], [883.0, 262.0]).len, 577.465);
}
#[test]
fn aurora_hggutubbpl() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([587.0, 460.0], [702.0, 76.0]).len, 572.758);
}
#[test]
fn aurora_ryakqhnhpi() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([81.0, 215.0], [614.0, 379.0]).len, 575.757);
}
#[test]
fn aurora_verrkjowcw() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([315.0, 53.0], [153.0, 513.0]).len, 580.12);
}
#[test]
fn aurora_xgmbdgyqsk() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([612.0, 421.0], [1014.0, 398.0]).len, 572.609);
}
#[test]
fn aurora_scurynmfza() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([835.0, 605.0], [276.0, 634.0]).len, 590.835);
}
#[test]
fn aurora_dnqoehbotx() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([458.0, 449.0], [975.0, 404.0]).len, 578.789);
}
#[test]
fn aurora_zvmbggmjds() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([701.0, 592.0], [419.0, 382.0]).len, 576.842);
}
#[test]
fn aurora_weuljcblei() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([355.0, 168.0], [684.0, 451.0]).len, 583.349);
}
#[test]
fn aurora_mpdcnhixwp() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([678.0, 449.0], [805.0, 89.0]).len, 585.723);
}
#[test]
fn aurora_rwpifzcvph() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([256.0, 633.0], [303.0, 290.0]).len, 583.831);
}
#[test]
fn aurora_lftpesbcbr() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([345.0, 553.0], [615.0, 323.0]).len, 586.466);
}
#[test]
fn aurora_uprjswozem() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([153.0, 150.0], [728.0, 54.0]).len, 595.294);
}
#[test]
fn aurora_bfzdwogncl() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([506.0, 373.0], [556.0, 684.0]).len, 578.543);
}
#[test]
fn aurora_uadnkfkmhn() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([531.0, 289.0], [971.0, 594.0]).len, 575.598);
}
#[test]
fn aurora_adysyvkeln() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([591.0, 119.0], [808.0, 441.0]).len, 578.632);
}
#[test]
fn aurora_bsigebokei() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([651.0, 426.0], [765.0, 77.0]).len, 584.352);
}
#[test]
fn aurora_kgfmvamnrg() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([136.0, 233.0], [646.0, 68.0]).len, 586.74);
}
#[test]
fn aurora_ofqqrptgpv() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([362.0, 218.0], [552.0, 566.0]).len, 594.669);
}
#[test]
fn aurora_jelbqukqpq() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([248.0, 79.0], [644.0, 400.0]).len, 584.427);
}
#[test]
fn aurora_lexknwvksq() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([68.0, 349.0], [632.0, 293.0]).len, 585.535);
}
#[test]
fn aurora_dnkadyoaho() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([649.0, 140.0], [472.0, 425.0]).len, 581.048);
}
#[test]
fn aurora_cdqywliqom() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([797.0, 297.0], [275.0, 68.0]).len, 585.36);
}
#[test]
fn aurora_vhvzqelzrz() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([89.0, 418.0], [655.0, 319.0]).len, 590.762);
}
#[test]
fn aurora_pblfngvffp() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([112.0, 151.0], [610.0, 293.0]).len, 595.009);
}
#[test]
fn aurora_sdrhyvfkez() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([158.0, 312.0], [461.0, 660.0]).len, 585.628);
}
#[test]
fn aurora_kazezynquq() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([676.0, 598.0], [464.0, 260.0]).len, 590.072);
}
#[test]
fn aurora_mnpvxhuyry() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([263.0, 350.0], [836.0, 319.0]).len, 592.106);
}
#[test]
fn aurora_xxofgubxts() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([565.0, 493.0], [745.0, 213.0]).len, 589.114);
}
#[test]
fn aurora_nkfjbapxlp() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([222.0, 366.0], [685.0, 84.0]).len, 581.74);
}
#[test]
fn aurora_njdhneavhb() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([497.0, 646.0], [952.0, 420.0]).len, 589.055);
}
#[test]
fn aurora_pgjlkmwozu() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([414.0, 436.0], [685.0, 117.0]).len, 588.387);
}
#[test]
fn aurora_qpohsnqosy() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([612.0, 230.0], [673.0, 630.0]).len, 591.951);
}
#[test]
fn aurora_vrjrgfwilv() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([240.0, 678.0], [458.0, 291.0]).len, 594.041);
}
#[test]
fn aurora_vgvhfgpxmg() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([244.0, 485.0], [810.0, 443.0]).len, 587.47);
}
#[test]
fn aurora_zbmqolcrdw() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([518.0, 167.0], [64.0, 477.0]).len, 587.684);
}
#[test]
fn aurora_nbqqdxtqeq() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([231.0, 427.0], [690.0, 669.0]).len, 592.132);
}
#[test]
fn aurora_rwkpmdzhks() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([211.0, 235.0], [715.0, 350.0]).len, 594.601);
}
#[test]
fn aurora_psyvopsnhm() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([855.0, 521.0], [317.0, 328.0]).len, 594.65);
}
#[test]
fn aurora_nbepzgioqj() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([638.0, 612.0], [454.0, 284.0]).len, 595.112);
}
#[test]
fn aurora_cdrhieeqda() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([324.0, 468.0], [897.0, 571.0]).len, 593.818);
}
#[test]
fn aurora_oqktjwgwrv() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([392.0, 637.0], [231.0, 291.0]).len, 593.518);
}
#[test]
fn aurora_yasgyvpife() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([236.0, 419.0], [790.0, 422.0]).len, 590.363);
}
#[test]
fn aurora_rspbwjhfxw() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([451.0, 507.0], [987.0, 406.0]).len, 595.91);
}
#[test]
fn aurora_mgofbmbcpb() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([409.0, 382.0], [784.0, 614.0]).len, 596.678);
}
#[test]
fn aurora_wmtcuxzcyq() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([40.0, 473.0], [379.0, 93.0]).len, 593.616);
}
#[test]
fn aurora_xzkgmkrqna() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([275.0, 530.0], [626.0, 294.0]).len, 600.581);
}
#[test]
fn aurora_ummoalamou() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([627.0, 589.0], [179.0, 405.0]).len, 599.901);
}
#[test]
fn aurora_einaeigonu() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([299.0, 152.0], [906.0, 145.0]).len, 613.826);
}
#[test]
fn aurora_txgosoydvs() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([356.0, 166.0], [661.0, 441.0]).len, 595.041);
}
#[test]
fn aurora_ymenrovsbf() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([129.0, 538.0], [527.0, 276.0]).len, 596.105);
}
#[test]
fn aurora_ksuoujvyeb() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([413.0, 419.0], [868.0, 297.0]).len, 593.492);
}
#[test]
fn aurora_sgpqphwdta() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([334.0, 459.0], [793.0, 286.0]).len, 600.659);
}
#[test]
fn aurora_lnvoydkqzx() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([554.0, 714.0], [9.0, 541.0]).len, 590.835);
}
#[test]
fn aurora_rqvpmeiyok() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([642.0, 516.0], [806.0, 185.0]).len, 596.485);
}
#[test]
fn aurora_gseehvusye() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([162.0, 376.0], [634.0, 234.0]).len, 594.587);
}
#[test]
fn aurora_zfszmprocq() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([459.0, 590.0], [985.0, 365.0]).len, 603.568);
}
#[test]
fn aurora_tjnhrzzibe() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([297.0, 158.0], [737.0, 404.0]).len, 601.47);
}
#[test]
fn aurora_stnmzenqcc() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([477.0, 225.0], [35.0, 543.0]).len, 607.805);
}
#[test]
fn aurora_qfobzjxeew() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([504.0, 319.0], [118.0, 593.0]).len, 602.211);
}
#[test]
fn aurora_vgafnpehzn() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([902.0, 540.0], [357.0, 354.0]).len, 598.287);
}
#[test]
fn aurora_hpqsbnzozi() {
    let aurora = aurora_mesh();
    known_failure!(aurora.path([827.0, 678.0], [460.0, 383.0]).len, 605.301);
}
#[test]
fn aurora_qyukdwiysm() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([412.0, 453.0], [692.0, 150.0]).len, 604.084);
}
#[test]
fn aurora_jmuurimyok() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([855.0, 236.0], [586.0, 606.0]).len, 603.812);
}
#[test]
fn aurora_ymmblqpkvf() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([233.0, 323.0], [422.0, 650.0]).len, 598.005);
}
#[test]
fn aurora_mtyuvygvcq() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([906.0, 161.0], [753.0, 566.0]).len, 603.828);
}
#[test]
fn aurora_ozuskvdbfg() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([468.0, 584.0], [280.0, 199.0]).len, 614.314);
}
#[test]
fn aurora_lytshcuond() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([120.0, 698.0], [424.0, 298.0]).len, 610.808);
}
#[test]
fn aurora_rjmttineah() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([962.0, 389.0], [427.0, 191.0]).len, 599.377);
}
#[test]
fn aurora_qlsulfhvht() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([512.0, 170.0], [480.0, 595.0]).len, 607.774);
}
#[test]
fn aurora_kjjxfemtdu() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([611.0, 658.0], [494.0, 282.0]).len, 604.497);
}
#[test]
fn aurora_nxzrwbmujr() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([782.0, 221.0], [203.0, 68.0]).len, 603.645);
}
#[test]
fn aurora_iivigfgtmj() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([911.0, 345.0], [564.0, 698.0]).len, 603.986);
}
#[test]
fn aurora_mhtrzuxtls() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([352.0, 267.0], [506.0, 633.0]).len, 613.012);
}
#[test]
fn aurora_yaonfmegrw() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([265.0, 408.0], [793.0, 622.0]).len, 599.604);
}
#[test]
fn aurora_pzxyeddpvg() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([705.0, 538.0], [176.0, 410.0]).len, 599.187);
}
#[test]
fn aurora_iockckfssv() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([548.0, 454.0], [32.0, 490.0]).len, 610.232);
}
#[test]
fn aurora_ljckcwvobn() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([572.0, 131.0], [522.0, 519.0]).len, 612.35);
}
#[test]
fn aurora_nkcahbyeww() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([708.0, 631.0], [139.0, 727.0]).len, 616.802);
}
#[test]
fn aurora_ezlprellwa() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([580.0, 412.0], [173.0, 182.0]).len, 605.375);
}
#[test]
fn aurora_pfkzidzcbs() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([622.0, 246.0], [694.0, 672.0]).len, 613.922);
}
#[test]
fn aurora_prjgokvskd() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([899.0, 313.0], [366.0, 67.0]).len, 607.405);
}
#[test]
fn aurora_ezvzcrqrli() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([391.0, 234.0], [210.0, 552.0]).len, 606.426);
}
#[test]
fn aurora_ojythaaxex() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([736.0, 318.0], [148.0, 391.0]).len, 612.109);
}
#[test]
fn aurora_wvwdvwqbxs() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([222.0, 472.0], [803.0, 563.0]).len, 608.266);
}
#[test]
fn aurora_qdkcpxxmdx() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([415.0, 668.0], [55.0, 317.0]).len, 610.401);
}
#[test]
fn aurora_auqpllvwij() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([358.0, 266.0], [873.0, 423.0]).len, 612.259);
}
#[test]
fn aurora_jzquqobagk() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([46.0, 398.0], [649.0, 323.0]).len, 620.905);
}
#[test]
fn aurora_aastdfvrjl() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([110.0, 513.0], [705.0, 564.0]).len, 616.64);
}
#[test]
fn aurora_qnodcqmxvh() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([203.0, 268.0], [766.0, 249.0]).len, 613.793);
}
#[test]
fn aurora_ybtpjtqqjm() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([641.0, 655.0], [42.0, 668.0]).len, 619.047);
}
#[test]
fn aurora_vgutrnaosc() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([252.0, 405.0], [826.0, 285.0]).len, 620.896);
}
#[test]
fn aurora_zbifmkhupk() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([692.0, 189.0], [217.0, 416.0]).len, 613.451);
}
#[test]
fn aurora_kwtwiimlys() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([308.0, 294.0], [265.0, 657.0]).len, 614.687);
}
#[test]
fn aurora_yydkyezlbn() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([676.0, 699.0], [885.0, 252.0]).len, 616.443);
}
#[test]
fn aurora_lzcbygneas() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([722.0, 428.0], [295.0, 150.0]).len, 609.251);
}
#[test]
fn aurora_dvaowhbegk() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([795.0, 103.0], [196.0, 229.0]).len, 618.207);
}
#[test]
fn aurora_ygorndpikv() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([513.0, 614.0], [560.0, 218.0]).len, 620.518);
}
#[test]
fn aurora_eqzdsodhtt() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([668.0, 601.0], [177.0, 283.0]).len, 611.961);
}
#[test]
fn aurora_pyawkazfit() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([894.0, 469.0], [453.0, 121.0]).len, 615.485);
}
#[test]
fn aurora_rnpdifikvi() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([627.0, 166.0], [420.0, 515.0]).len, 620.628);
}
#[test]
fn aurora_ipejpzzstx() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([665.0, 271.0], [235.0, 512.0]).len, 621.887);
}
#[test]
fn aurora_mqbueqapia() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([142.0, 272.0], [662.0, 564.0]).len, 613.323);
}
#[test]
fn aurora_uugayuthyh() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([793.0, 624.0], [394.0, 300.0]).len, 612.886);
}
#[test]
fn aurora_tjrdtbbdzj() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([282.0, 344.0], [888.0, 274.0]).len, 626.291);
}
#[test]
fn aurora_urorrejfqe() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([974.0, 283.0], [595.0, 666.0]).len, 618.25);
}
#[test]
fn aurora_gbixevkxdw() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([725.0, 140.0], [210.0, 368.0]).len, 618.698);
}
#[test]
fn aurora_jqvshovfci() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([511.0, 661.0], [526.0, 277.0]).len, 619.247);
}
#[test]
fn aurora_lovuvyrpgr() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([450.0, 596.0], [882.0, 252.0]).len, 629.061);
}
#[test]
fn aurora_sthlexgkqd() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([511.0, 341.0], [890.0, 667.0]).len, 619.387);
}
#[test]
fn aurora_dphtmkkfjw() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([442.0, 316.0], [686.0, 634.0]).len, 625.555);
}
#[test]
fn aurora_mtqdvrxacq() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([8.0, 375.0], [550.0, 652.0]).len, 615.799);
}
#[test]
fn aurora_jgvcwebewm() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([814.0, 420.0], [308.0, 268.0]).len, 621.068);
}
#[test]
fn aurora_bfacwhmlqm() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([363.0, 115.0], [659.0, 456.0]).len, 622.082);
}
#[test]
fn aurora_jrtqntgvxk() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([794.0, 409.0], [318.0, 487.0]).len, 617.266);
}
#[test]
fn aurora_xybcvpzafe() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([419.0, 391.0], [866.0, 132.0]).len, 629.989);
}
#[test]
fn aurora_ekjmzotuai() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([131.0, 116.0], [730.0, 60.0]).len, 623.167);
}
#[test]
fn aurora_pjctmueznu() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([640.0, 665.0], [877.0, 255.0]).len, 625.475);
}
#[test]
fn aurora_axsedoksms() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([578.0, 434.0], [16.0, 187.0]).len, 623.406);
}
#[test]
fn aurora_nyeppoemru() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([234.0, 220.0], [836.0, 258.0]).len, 633.422);
}
#[test]
fn aurora_jpkqbtxtwh() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([587.0, 661.0], [939.0, 265.0]).len, 623.862);
}
#[test]
fn aurora_vdocbqmqov() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([609.0, 453.0], [615.0, 742.0]).len, 633.342);
}
#[test]
fn aurora_trpqpbayfc() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([131.0, 571.0], [316.0, 151.0]).len, 628.187);
}
#[test]
fn aurora_pjtntaqyud() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([210.0, 143.0], [598.0, 424.0]).len, 624.858);
}
#[test]
fn aurora_okygrajcct() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([476.0, 253.0], [312.0, 667.0]).len, 625.925);
}
#[test]
fn aurora_hilmasundy() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([317.0, 156.0], [90.0, 555.0]).len, 626.77);
}
#[test]
fn aurora_qmagjvqvhq() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([257.0, 494.0], [747.0, 317.0]).len, 628.169);
}
#[test]
fn aurora_shedsfajhe() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([726.0, 614.0], [134.0, 606.0]).len, 632.218);
}
#[test]
fn aurora_twmzfvzsoj() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([850.0, 303.0], [261.0, 396.0]).len, 635.639);
}
#[test]
fn aurora_woptbzluxo() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([744.0, 366.0], [245.0, 474.0]).len, 625.91);
}
#[test]
fn aurora_zisxudgaxu() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([171.0, 310.0], [726.0, 554.0]).len, 624.069);
}
#[test]
fn aurora_fvfdvadkim() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([366.0, 293.0], [843.0, 592.0]).len, 624.303);
}
#[test]
fn aurora_anvoninovf() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([866.0, 373.0], [457.0, 660.0]).len, 633.782);
}
#[test]
fn aurora_jajnoqrvyq() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([706.0, 104.0], [85.0, 123.0]).len, 643.778);
}
#[test]
fn aurora_ubpafaqbpt() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([901.0, 552.0], [285.0, 513.0]).len, 631.642);
}
#[test]
fn aurora_mxahekdahn() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([29.0, 538.0], [558.0, 415.0]).len, 629.094);
}
#[test]
fn aurora_rfgyedgwij() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([375.0, 194.0], [448.0, 590.0]).len, 642.373);
}
#[test]
fn aurora_zvzjjcwuyw() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([256.0, 496.0], [675.0, 412.0]).len, 622.852);
}
#[test]
fn aurora_chpbgehnpn() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([651.0, 111.0], [858.0, 488.0]).len, 629.438);
}
#[test]
fn aurora_pahelphhjn() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([429.0, 551.0], [248.0, 134.0]).len, 642.785);
}
#[test]
fn aurora_aqeqihswtq() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([206.0, 138.0], [271.0, 582.0]).len, 633.297);
}
#[test]
fn aurora_mqvvmnfiss() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([714.0, 173.0], [68.0, 138.0]).len, 654.47);
}
#[test]
fn aurora_isaspxyouw() {
    let aurora = aurora_mesh();
    known_failure!(aurora.path([903.0, 594.0], [537.0, 196.0]).len, 632.631);
}
#[test]
fn aurora_mqdmpykewl() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([638.0, 561.0], [66.0, 489.0]).len, 635.457);
}
#[test]
fn aurora_pzrtgzqlbu() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([845.0, 289.0], [634.0, 739.0]).len, 641.438);
}
#[test]
fn aurora_ztkmotbyqb() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([624.0, 519.0], [37.0, 399.0]).len, 629.169);
}
#[test]
fn aurora_fispsytdpm() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([228.0, 586.0], [387.0, 237.0]).len, 631.991);
}
#[test]
fn aurora_wdiewfmbmz() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([148.0, 488.0], [767.0, 420.0]).len, 638.24);
}
#[test]
fn aurora_gpgqtijhjr() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([474.0, 303.0], [207.0, 700.0]).len, 643.417);
}
#[test]
fn aurora_vnzlhnlfpy() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([20.0, 403.0], [624.0, 473.0]).len, 635.691);
}
#[test]
fn aurora_oulfhlgqpi() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([279.0, 291.0], [638.0, 601.0]).len, 629.479);
}
#[test]
fn aurora_ahbqoccjtd() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([127.0, 488.0], [715.0, 390.0]).len, 639.701);
}
#[test]
fn aurora_pudjgimkbm() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([764.0, 364.0], [251.0, 56.0]).len, 631.746);
}
#[test]
fn aurora_yozhnnyrdy() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([484.0, 399.0], [702.0, 702.0]).len, 639.465);
}
#[test]
fn aurora_qljcueqbbi() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([876.0, 296.0], [275.0, 389.0]).len, 642.142);
}
#[test]
fn aurora_rkdsokbpvy() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([112.0, 521.0], [481.0, 212.0]).len, 642.266);
}
#[test]
fn aurora_habhwagmjl() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([316.0, 676.0], [847.0, 516.0]).len, 640.527);
}
#[test]
fn aurora_dblbxsyide() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([126.0, 361.0], [702.0, 168.0]).len, 653.175);
}
#[test]
fn aurora_gvobtdbvic() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([383.0, 632.0], [279.0, 250.0]).len, 636.415);
}
#[test]
fn aurora_jagepkvntc() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([238.0, 716.0], [821.0, 495.0]).len, 648.683);
}
#[test]
fn aurora_dimgnzwmsc() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([629.0, 591.0], [5.0, 648.0]).len, 661.471);
}
#[test]
fn aurora_oamxnhncex() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([303.0, 263.0], [109.0, 669.0]).len, 645.588);
}
#[test]
fn aurora_nzhoxykymx() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([837.0, 100.0], [304.0, 324.0]).len, 634.713);
}
#[test]
fn aurora_cijfllwouu() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([350.0, 454.0], [930.0, 654.0]).len, 635.923);
}
#[test]
fn aurora_xkweeppylr() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([51.0, 521.0], [620.0, 339.0]).len, 645.499);
}
#[test]
fn aurora_qvtnsmvjjk() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([712.0, 187.0], [157.0, 358.0]).len, 647.49);
}
#[test]
fn aurora_pgnoyauvzm() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([557.0, 691.0], [548.0, 452.0]).len, 643.226);
}
#[test]
fn aurora_bgnxivgrxk() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([169.0, 150.0], [784.0, 313.0]).len, 645.478);
}
#[test]
fn aurora_rgpakqdwfz() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([111.0, 439.0], [608.0, 678.0]).len, 635.395);
}
#[test]
fn aurora_lrijhexmcq() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([286.0, 85.0], [230.0, 524.0]).len, 648.844);
}
#[test]
fn aurora_sjdilwnsnf() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([991.0, 444.0], [446.0, 422.0]).len, 639.1);
}
#[test]
fn aurora_mayowcqipu() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([501.0, 210.0], [1020.0, 442.0]).len, 639.91);
}
#[test]
fn aurora_ywwvzecrgk() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([909.0, 245.0], [309.0, 372.0]).len, 646.55);
}
#[test]
fn aurora_pmytqidxgq() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([302.0, 163.0], [621.0, 498.0]).len, 636.542);
}
#[test]
fn aurora_uozsfwzxel() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([96.0, 182.0], [494.0, 488.0]).len, 643.572);
}
#[test]
fn aurora_mhlftnlwyl() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([783.0, 228.0], [152.0, 217.0]).len, 655.307);
}
#[test]
fn aurora_teoyiuauld() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([819.0, 155.0], [386.0, 386.0]).len, 653.198);
}
#[test]
fn aurora_emqoboqjbe() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([413.0, 333.0], [668.0, 671.0]).len, 647.926);
}
#[test]
fn aurora_esfuzckece() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([617.0, 501.0], [393.0, 248.0]).len, 641.224);
}
#[test]
fn aurora_npalzrsafx() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([564.0, 548.0], [221.0, 231.0]).len, 650.782);
}
#[test]
fn aurora_budblzerql() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([232.0, 228.0], [575.0, 554.0]).len, 652.04);
}
#[test]
fn aurora_ashopdfwgb() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([558.0, 259.0], [283.0, 612.0]).len, 644.766);
}
#[test]
fn aurora_plrootjuzo() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([377.0, 133.0], [321.0, 525.0]).len, 651.082);
}
#[test]
fn aurora_uahevfjfhy() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([331.0, 266.0], [694.0, 554.0]).len, 641.774);
}
#[test]
fn aurora_uhzxdoylth() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([274.0, 587.0], [598.0, 267.0]).len, 648.238);
}
#[test]
fn aurora_zxlwexopts() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([769.0, 326.0], [143.0, 248.0]).len, 650.506);
}
#[test]
fn aurora_xivrcgfjwa() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([608.0, 673.0], [12.0, 496.0]).len, 642.854);
}
#[test]
fn aurora_boxlajmbkp() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([663.0, 650.0], [24.0, 678.0]).len, 659.428);
}
#[test]
fn aurora_asynadgaas() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([879.0, 303.0], [274.0, 124.0]).len, 643.704);
}
#[test]
fn aurora_ctqrkkeywl() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([345.0, 230.0], [61.0, 637.0]).len, 653.937);
}
#[test]
fn aurora_ablxpyxxzm() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([124.0, 549.0], [439.0, 184.0]).len, 646.148);
}
#[test]
fn aurora_zlaltoeqvb() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([858.0, 434.0], [346.0, 487.0]).len, 648.679);
}
#[test]
fn aurora_jdawjfgsip() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([339.0, 164.0], [603.0, 532.0]).len, 646.767);
}
#[test]
fn aurora_mnmcxqlbpv() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([750.0, 696.0], [226.0, 681.0]).len, 649.555);
}
#[test]
fn aurora_kluejvkxtf() {
    let aurora = aurora_mesh();
    known_failure!(aurora.path([815.0, 693.0], [473.0, 340.0]).len, 651.111);
}
#[test]
fn aurora_plobhsusxn() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([854.0, 143.0], [574.0, 493.0]).len, 650.665);
}
#[test]
fn aurora_bzicxjzvzc() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([577.0, 591.0], [475.0, 172.0]).len, 654.576);
}
#[test]
fn aurora_esmoveactu() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([646.0, 304.0], [26.0, 188.0]).len, 655.421);
}
#[test]
fn aurora_yqhmzehnse() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([274.0, 235.0], [880.0, 235.0]).len, 655.593);
}
#[test]
fn aurora_hayhylrahz() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([333.0, 467.0], [207.0, 73.0]).len, 653.804);
}
#[test]
fn aurora_biuoyxtlwx() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([154.0, 369.0], [757.0, 396.0]).len, 651.763);
}
#[test]
fn aurora_ejhoaiawdb() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([768.0, 284.0], [711.0, 607.0]).len, 656.174);
}
#[test]
fn aurora_xqxyfpqdvl() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([858.0, 273.0], [408.0, 618.0]).len, 656.619);
}
#[test]
fn aurora_jqnadfwlsn() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([960.0, 551.0], [626.0, 202.0]).len, 656.165);
}
#[test]
fn aurora_vikkdyucby() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([289.0, 122.0], [522.0, 569.0]).len, 656.577);
}
#[test]
fn aurora_ltjdqpreon() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([66.0, 206.0], [354.0, 662.0]).len, 661.249);
}
#[test]
fn aurora_ahiehvlies() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([121.0, 192.0], [549.0, 473.0]).len, 649.779);
}
#[test]
fn aurora_uuabmqbilr() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([324.0, 148.0], [924.0, 375.0]).len, 658.641);
}
#[test]
fn aurora_osxsrgyzye() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([557.0, 548.0], [263.0, 133.0]).len, 664.913);
}
#[test]
fn aurora_vuagreccvz() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([250.0, 223.0], [882.0, 311.0]).len, 660.583);
}
#[test]
fn aurora_ycwdwrklix() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([343.0, 110.0], [520.0, 519.0]).len, 663.48);
}
#[test]
fn aurora_ojuyamsyyq() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([896.0, 364.0], [299.0, 396.0]).len, 668.357);
}
#[test]
fn aurora_jfiwmoiydt() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([403.0, 648.0], [466.0, 353.0]).len, 656.348);
}
#[test]
fn aurora_xflmdvxaaz() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([898.0, 368.0], [288.0, 182.0]).len, 665.376);
}
#[test]
fn aurora_awwhqgnigh() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([805.0, 505.0], [645.0, 146.0]).len, 655.025);
}
#[test]
fn aurora_bxocowsaoa() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([25.0, 358.0], [630.0, 171.0]).len, 660.694);
}
#[test]
fn aurora_tufpbknmxi() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([401.0, 230.0], [196.0, 593.0]).len, 660.711);
}
#[test]
fn aurora_wtwjcltmtj() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([391.0, 713.0], [297.0, 367.0]).len, 668.91);
}
#[test]
fn aurora_grcnzwjrfj() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([345.0, 191.0], [953.0, 347.0]).len, 653.09);
}
#[test]
fn aurora_dsewlsubxh() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([780.0, 327.0], [368.0, 671.0]).len, 669.915);
}
#[test]
fn aurora_meeancjjun() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([680.0, 599.0], [41.0, 626.0]).len, 678.015);
}
#[test]
fn aurora_nkjadvywjc() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([944.0, 657.0], [300.0, 624.0]).len, 665.041);
}
#[test]
fn aurora_ukbouhnxko() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([391.0, 430.0], [740.0, 248.0]).len, 659.051);
}
#[test]
fn aurora_cmoqzinfyj() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([206.0, 151.0], [850.0, 182.0]).len, 671.16);
}
#[test]
fn aurora_stikwhhlig() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([304.0, 575.0], [959.0, 638.0]).len, 671.046);
}
#[test]
fn aurora_iptognnmbt() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([190.0, 359.0], [717.0, 45.0]).len, 657.756);
}
#[test]
fn aurora_xtflgepydw() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([513.0, 311.0], [239.0, 698.0]).len, 668.062);
}
#[test]
fn aurora_ergksvwaxy() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([487.0, 399.0], [218.0, 697.0]).len, 672.813);
}
#[test]
fn aurora_zpklhtiuin() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([32.0, 182.0], [664.0, 310.0]).len, 668.572);
}
#[test]
fn aurora_rqfyooogcw() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([162.0, 446.0], [773.0, 622.0]).len, 661.114);
}
#[test]
fn aurora_kecsrnfoub() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([260.0, 200.0], [901.0, 312.0]).len, 667.948);
}
#[test]
fn aurora_exypljwodv() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([222.0, 568.0], [225.0, 81.0]).len, 666.445);
}
#[test]
fn aurora_kturtqhxsj() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([202.0, 205.0], [278.0, 663.0]).len, 668.332);
}
#[test]
fn aurora_mluetmcssc() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([414.0, 608.0], [400.0, 241.0]).len, 667.676);
}
#[test]
fn aurora_nyaylyectj() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([176.0, 169.0], [750.0, 344.0]).len, 666.794);
}
#[test]
fn aurora_ajykmhsjtq() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([734.0, 212.0], [103.0, 123.0]).len, 659.807);
}
#[test]
fn aurora_cgzaygnbgm() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([770.0, 564.0], [842.0, 124.0]).len, 673.305);
}
#[test]
fn aurora_hmhbdfxoeb() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([618.0, 540.0], [853.0, 151.0]).len, 667.329);
}
#[test]
fn aurora_nicydlmesx() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([286.0, 89.0], [415.0, 544.0]).len, 672.039);
}
#[test]
fn aurora_fanadfcoys() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([174.0, 293.0], [642.0, 657.0]).len, 671.255);
}
#[test]
fn aurora_jarugefkpf() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([387.0, 439.0], [932.0, 340.0]).len, 670.096);
}
#[test]
fn aurora_aluvipddvs() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([730.0, 654.0], [198.0, 282.0]).len, 666.629);
}
#[test]
fn aurora_wkqwvgtxns() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([879.0, 424.0], [283.0, 172.0]).len, 670.186);
}
#[test]
fn aurora_hxbdvwavjr() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([463.0, 390.0], [47.0, 655.0]).len, 676.792);
}
#[test]
fn aurora_zcpxfwsnhl() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([345.0, 672.0], [696.0, 395.0]).len, 671.079);
}
#[test]
fn aurora_saxpovkhrh() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([423.0, 464.0], [1014.0, 331.0]).len, 675.96);
}
#[test]
fn aurora_ahnfsmhskl() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([402.0, 139.0], [1009.0, 375.0]).len, 673.38);
}
#[test]
fn aurora_sqlfkzgtmm() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([189.0, 622.0], [349.0, 179.0]).len, 672.533);
}
#[test]
fn aurora_kjhoonfjpb() {
    let aurora = aurora_mesh();
    known_failure!(aurora.path([27.0, 581.0], [266.0, 193.0]).len, 671.221);
}
#[test]
fn aurora_tojhmxjlnz() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([798.0, 615.0], [228.0, 398.0]).len, 675.316);
}
#[test]
fn aurora_lsmvffbaop() {
    let aurora = aurora_mesh();
    known_failure!(aurora.path([776.0, 671.0], [470.0, 303.0]).len, 670.519);
}
#[test]
fn aurora_vfxjfrphgm() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([39.0, 541.0], [608.0, 381.0]).len, 672.232);
}
#[test]
fn aurora_nrapkauxci() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([683.0, 613.0], [53.0, 491.0]).len, 673.844);
}
#[test]
fn aurora_sjczanleiq() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([134.0, 503.0], [658.0, 719.0]).len, 668.871);
}
#[test]
fn aurora_limivsreep() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([454.0, 268.0], [770.0, 640.0]).len, 670.215);
}
#[test]
fn aurora_fvvmeykumq() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([511.0, 202.0], [156.0, 640.0]).len, 677.233);
}
#[test]
fn aurora_momorwjntm() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([866.0, 173.0], [475.0, 581.0]).len, 682.586);
}
#[test]
fn aurora_ymsoksihyy() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([811.0, 185.0], [935.0, 588.0]).len, 677.211);
}
#[test]
fn aurora_ftwcjmbcfa() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([792.0, 145.0], [810.0, 605.0]).len, 680.471);
}
#[test]
fn aurora_vdumadvvsm() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([232.0, 698.0], [740.0, 401.0]).len, 676.492);
}
#[test]
fn aurora_jesqyhqexd() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([902.0, 444.0], [307.0, 734.0]).len, 681.434);
}
#[test]
fn aurora_nhqdypwysk() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([527.0, 562.0], [317.0, 92.0]).len, 683.475);
}
#[test]
fn aurora_mbkgpjglbg() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([679.0, 711.0], [501.0, 290.0]).len, 678.346);
}
#[test]
fn aurora_upyexayfay() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([507.0, 395.0], [283.0, 692.0]).len, 676.487);
}
#[test]
fn aurora_khvuzeoyhu() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([523.0, 155.0], [137.0, 546.0]).len, 676.999);
}
#[test]
fn aurora_qovvugyxia() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([936.0, 345.0], [372.0, 475.0]).len, 683.141);
}
#[test]
fn aurora_kjpllgpkub() {
    let aurora = aurora_mesh();
    known_failure!(aurora.path([806.0, 501.0], [618.0, 102.0]).len, 676.816);
}
#[test]
fn aurora_nndxgqgack() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([564.0, 179.0], [730.0, 513.0]).len, 672.945);
}
#[test]
fn aurora_cahbxztaro() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([765.0, 690.0], [204.0, 510.0]).len, 677.596);
}
#[test]
fn aurora_ohdslbnoyt() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([828.0, 178.0], [151.0, 176.0]).len, 695.05);
}
#[test]
fn aurora_yqybwpkpjp() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([96.0, 231.0], [593.0, 610.0]).len, 685.515);
}
#[test]
fn aurora_qkaiqvebwu() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([256.0, 198.0], [159.0, 648.0]).len, 682.829);
}
#[test]
fn aurora_vkofvlidep() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([644.0, 451.0], [246.0, 181.0]).len, 676.794);
}
#[test]
fn aurora_drdmtgamyu() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([647.0, 232.0], [37.0, 349.0]).len, 681.545);
}
#[test]
fn aurora_dbevwxrlka() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([286.0, 141.0], [145.0, 601.0]).len, 677.933);
}
#[test]
fn aurora_mxndbnlrra() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([636.0, 252.0], [256.0, 568.0]).len, 679.307);
}
#[test]
fn aurora_lggxgofdzn() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([330.0, 235.0], [623.0, 608.0]).len, 686.662);
}
#[test]
fn aurora_zsxdevqgir() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([414.0, 702.0], [890.0, 359.0]).len, 690.577);
}
#[test]
fn aurora_ldazgphidf() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([169.0, 556.0], [827.0, 613.0]).len, 683.494);
}
#[test]
fn aurora_khzcjbkyqc() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([310.0, 522.0], [913.0, 413.0]).len, 680.118);
}
#[test]
fn aurora_kgbxczdpgo() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([1001.0, 336.0], [405.0, 489.0]).len, 684.693);
}
#[test]
fn aurora_olertzjkhf() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([63.0, 537.0], [652.0, 339.0]).len, 692.85);
}
#[test]
fn aurora_xqghcehfws() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([936.0, 662.0], [268.0, 617.0]).len, 689.424);
}
#[test]
fn aurora_qbvacqblqk() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([631.0, 638.0], [338.0, 286.0]).len, 689.769);
}
#[test]
fn aurora_xpnsktjrpo() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([134.0, 571.0], [213.0, 59.0]).len, 681.562);
}
#[test]
fn aurora_vdrxfswcba() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([176.0, 660.0], [541.0, 202.0]).len, 686.274);
}
#[test]
fn aurora_knvvabphqz() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([942.0, 627.0], [326.0, 340.0]).len, 684.397);
}
#[test]
fn aurora_aduqdwlghi() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([27.0, 180.0], [213.0, 678.0]).len, 697.061);
}
#[test]
fn aurora_vbszinayyj() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([700.0, 249.0], [46.0, 162.0]).len, 699.122);
}
#[test]
fn aurora_dlmbdlkcnd() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([825.0, 109.0], [132.0, 174.0]).len, 702.454);
}
#[test]
fn aurora_zqwmiebhvj() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([528.0, 714.0], [173.0, 289.0]).len, 694.684);
}
#[test]
fn aurora_pkesibcsvz() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([491.0, 613.0], [21.0, 188.0]).len, 694.214);
}
#[test]
fn aurora_gskmistyph() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([505.0, 656.0], [132.0, 231.0]).len, 697.758);
}
#[test]
fn aurora_cjwrtaolml() {
    let aurora = aurora_mesh();
    known_failure!(aurora.path([803.0, 686.0], [232.0, 424.0]).len, 683.383);
}
#[test]
fn aurora_osequihhnn() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([996.0, 412.0], [363.0, 510.0]).len, 694.416);
}
#[test]
fn aurora_jjnwthffyz() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([57.0, 138.0], [127.0, 629.0]).len, 694.438);
}
#[test]
fn aurora_olwgnivgbv() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([689.0, 256.0], [528.0, 457.0]).len, 687.42);
}
#[test]
fn aurora_gonhguelhi() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([319.0, 620.0], [356.0, 228.0]).len, 690.749);
}
#[test]
fn aurora_zrjgslzaod() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([487.0, 581.0], [148.0, 157.0]).len, 696.203);
}
#[test]
fn aurora_tbldhvgljg() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([435.0, 636.0], [398.0, 212.0]).len, 696.656);
}
#[test]
fn aurora_zhbblbrhqc() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([801.0, 372.0], [183.0, 112.0]).len, 692.669);
}
#[test]
fn aurora_udbbcjzhda() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([321.0, 232.0], [637.0, 601.0]).len, 684.886);
}
#[test]
fn aurora_tyvcpqhiml() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([1018.0, 422.0], [424.0, 145.0]).len, 688.184);
}
#[test]
fn aurora_zcqzhlrlkl() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([767.0, 603.0], [141.0, 431.0]).len, 695.197);
}
#[test]
fn aurora_hqjvspmeym() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([902.0, 321.0], [243.0, 188.0]).len, 691.174);
}
#[test]
fn aurora_syrkhcauja() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([264.0, 701.0], [171.0, 238.0]).len, 691.11);
}
#[test]
fn aurora_hbccnikcel() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([643.0, 690.0], [85.0, 551.0]).len, 694.865);
}
#[test]
fn aurora_jpzkrjuxkh() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([725.0, 222.0], [839.0, 621.0]).len, 695.528);
}
#[test]
fn aurora_ukzfjsxjkc() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([747.0, 94.0], [171.0, 364.0]).len, 688.49);
}
#[test]
fn aurora_jewxbdvoig() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([83.0, 184.0], [579.0, 443.0]).len, 692.901);
}
#[test]
fn aurora_qqurbbcuho() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([738.0, 78.0], [433.0, 470.0]).len, 692.834);
}
#[test]
fn aurora_lfdqvtolkn() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([282.0, 521.0], [636.0, 120.0]).len, 693.04);
}
#[test]
fn aurora_oipfjnrgfz() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([656.0, 455.0], [0.0, 571.0]).len, 690.596);
}
#[test]
fn aurora_hjuhfbccrs() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([18.0, 194.0], [586.0, 576.0]).len, 699.407);
}
#[test]
fn aurora_lwfrdfkilu() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([597.0, 32.0], [907.0, 493.0]).len, 697.264);
}
#[test]
fn aurora_jlpbrkhedq() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([32.0, 561.0], [668.0, 620.0]).len, 695.396);
}
#[test]
fn aurora_tyggfkckti() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([124.0, 316.0], [763.0, 555.0]).len, 698.124);
}
#[test]
fn aurora_yrqmvrzvbg() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([385.0, 727.0], [657.0, 304.0]).len, 692.112);
}
#[test]
fn aurora_bmsebqpdxb() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([91.0, 573.0], [569.0, 398.0]).len, 695.415);
}
#[test]
fn aurora_bnzgutzrgk() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([682.0, 677.0], [60.0, 554.0]).len, 698.146);
}
#[test]
fn aurora_hhoxcbmvex() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([967.0, 353.0], [421.0, 708.0]).len, 703.533);
}
#[test]
fn aurora_ifbnfxtgou() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([428.0, 658.0], [516.0, 351.0]).len, 696.039);
}
#[test]
fn aurora_gqgtyxvxaj() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([142.0, 504.0], [821.0, 549.0]).len, 700.64);
}
#[test]
fn aurora_zzidgqyyrf() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([176.0, 699.0], [846.0, 689.0]).len, 700.941);
}
#[test]
fn aurora_pivmhcpboj() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([146.0, 650.0], [135.0, 141.0]).len, 703.551);
}
#[test]
fn aurora_najrskuemu() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([684.0, 696.0], [483.0, 239.0]).len, 702.649);
}
#[test]
fn aurora_tudtydfncg() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([461.0, 468.0], [724.0, 252.0]).len, 699.606);
}
#[test]
fn aurora_qsvoeduubd() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([228.0, 587.0], [287.0, 89.0]).len, 707.808);
}
#[test]
fn aurora_taxipdpcwa() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([321.0, 344.0], [1014.0, 316.0]).len, 712.769);
}
#[test]
fn aurora_ijjeacwofj() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([623.0, 329.0], [313.0, 605.0]).len, 710.605);
}
#[test]
fn aurora_lorfoipguy() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([428.0, 397.0], [663.0, 706.0]).len, 713.81);
}
#[test]
fn aurora_iwyhgueioi() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([518.0, 260.0], [71.0, 657.0]).len, 705.892);
}
#[test]
fn aurora_ntuqlppdbb() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([977.0, 438.0], [398.0, 130.0]).len, 702.47);
}
#[test]
fn aurora_vnxdolthoj() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([162.0, 673.0], [197.0, 136.0]).len, 708.689);
}
#[test]
fn aurora_yiagefkgsw() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([138.0, 682.0], [156.0, 161.0]).len, 707.073);
}
#[test]
fn aurora_aztxeynysx() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([978.0, 398.0], [329.0, 347.0]).len, 701.174);
}
#[test]
fn aurora_htjouaiigl() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([147.0, 658.0], [798.0, 479.0]).len, 722.778);
}
#[test]
fn aurora_pafeewdysn() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([723.0, 59.0], [825.0, 517.0]).len, 704.514);
}
#[test]
fn aurora_ygisthhslq() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([101.0, 331.0], [749.0, 537.0]).len, 703.759);
}
#[test]
fn aurora_drttmhogmw() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([564.0, 599.0], [400.0, 176.0]).len, 712.55);
}
#[test]
fn aurora_ciyetpjeer() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([422.0, 617.0], [438.0, 221.0]).len, 712.313);
}
#[test]
fn aurora_rfcfkraimm() {
    let aurora = aurora_mesh();
    known_failure!(aurora.path([752.0, 609.0], [486.0, 146.0]).len, 706.424);
}
#[test]
fn aurora_xpfunzhuyi() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([64.0, 150.0], [625.0, 386.0]).len, 700.56);
}
#[test]
fn aurora_ndgvvkmrtr() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([208.0, 490.0], [695.0, 220.0]).len, 700.359);
}
#[test]
fn aurora_todboulgai() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([397.0, 209.0], [278.0, 636.0]).len, 704.343);
}
#[test]
fn aurora_wjfjwhriet() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([862.0, 576.0], [645.0, 193.0]).len, 712.266);
}
#[test]
fn aurora_ueaddmuisj() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([552.0, 557.0], [803.0, 136.0]).len, 712.013);
}
#[test]
fn aurora_mkuiilrwqk() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([447.0, 399.0], [323.0, 716.0]).len, 717.968);
}
#[test]
fn aurora_kpysteiexy() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([75.0, 290.0], [740.0, 238.0]).len, 716.383);
}
#[test]
fn aurora_dinffheutn() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([309.0, 191.0], [595.0, 654.0]).len, 715.922);
}
#[test]
fn aurora_kjjxsvqfqc() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([126.0, 559.0], [243.0, 26.0]).len, 707.503);
}
#[test]
fn aurora_flazljsoec() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([202.0, 584.0], [439.0, 148.0]).len, 711.178);
}
#[test]
fn aurora_rzfynxdocu() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([766.0, 209.0], [204.0, 433.0]).len, 708.887);
}
#[test]
fn aurora_lnsmqxcaqb() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([262.0, 314.0], [891.0, 623.0]).len, 704.644);
}
#[test]
fn aurora_dqxywhsnwo() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([330.0, 50.0], [298.0, 538.0]).len, 713.444);
}
#[test]
fn aurora_oiktqstspp() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([145.0, 400.0], [723.0, 99.0]).len, 699.759);
}
#[test]
fn aurora_ojkkupccbb() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([422.0, 233.0], [841.0, 467.0]).len, 710.718);
}
#[test]
fn aurora_ducsucdkox() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([206.0, 585.0], [272.0, 67.0]).len, 722.026);
}
#[test]
fn aurora_bsqsejzflv() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([660.0, 536.0], [750.0, 265.0]).len, 716.548);
}
#[test]
fn aurora_uaucaoohmw() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([844.0, 205.0], [378.0, 544.0]).len, 714.032);
}
#[test]
fn aurora_mvaaqxabii() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([761.0, 423.0], [178.0, 676.0]).len, 712.05);
}
#[test]
fn aurora_axvljwwczl() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([642.0, 430.0], [207.0, 569.0]).len, 709.976);
}
#[test]
fn aurora_whxxjnwqma() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([217.0, 438.0], [849.0, 392.0]).len, 715.351);
}
#[test]
fn aurora_alwnlukekg() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([848.0, 561.0], [805.0, 77.0]).len, 715.403);
}
#[test]
fn aurora_pzakbqijyz() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([347.0, 181.0], [564.0, 646.0]).len, 716.683);
}
#[test]
fn aurora_uxluumqkwb() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([385.0, 634.0], [606.0, 266.0]).len, 714.728);
}
#[test]
fn aurora_xiafdujuyi() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([826.0, 181.0], [142.0, 265.0]).len, 728.65);
}
#[test]
fn aurora_uxgdrsfhhg() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([153.0, 487.0], [791.0, 346.0]).len, 723.459);
}
#[test]
fn aurora_jcctgtytge() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([828.0, 619.0], [142.0, 558.0]).len, 715.737);
}
#[test]
fn aurora_vxxlhvclww() {
    let aurora = aurora_mesh();
    known_failure!(aurora.path([748.0, 604.0], [504.0, 212.0]).len, 712.14);
}
#[test]
fn aurora_rpdrjsmmev() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([818.0, 551.0], [766.0, 64.0]).len, 720.59);
}
#[test]
fn aurora_tnxiuhekrn() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([142.0, 436.0], [729.0, 132.0]).len, 717.587);
}
#[test]
fn aurora_wlejvdoopp() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([447.0, 80.0], [807.0, 532.0]).len, 715.198);
}
#[test]
fn aurora_zjscenvvrf() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([825.0, 583.0], [746.0, 100.0]).len, 723.068);
}
#[test]
fn aurora_rvqyfupzoa() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([746.0, 122.0], [887.0, 571.0]).len, 717.993);
}
#[test]
fn aurora_mmlmwkeizw() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([680.0, 640.0], [887.0, 164.0]).len, 724.58);
}
#[test]
fn aurora_tadjvxdani() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([184.0, 86.0], [818.0, 380.0]).len, 716.259);
}
#[test]
fn aurora_pybbistqdo() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([888.0, 398.0], [205.0, 343.0]).len, 721.585);
}
#[test]
fn aurora_mnfqwvpyxa() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([960.0, 409.0], [369.0, 92.0]).len, 715.224);
}
#[test]
fn aurora_etlhkgwbec() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([871.0, 244.0], [180.0, 320.0]).len, 724.339);
}
#[test]
fn aurora_cxtbjwxvft() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([139.0, 704.0], [335.0, 214.0]).len, 721.547);
}
#[test]
fn aurora_aeingufepi() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([759.0, 152.0], [70.0, 307.0]).len, 733.942);
}
#[test]
fn aurora_fefqdtgzhr() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([43.0, 579.0], [665.0, 696.0]).len, 726.932);
}
#[test]
fn aurora_vvyxtgfrms() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([584.0, 201.0], [433.0, 611.0]).len, 726.876);
}
#[test]
fn aurora_wuwgpgxfac() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([76.0, 616.0], [120.0, 119.0]).len, 723.932);
}
#[test]
fn aurora_nbuffasxyp() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([790.0, 203.0], [757.0, 683.0]).len, 724.515);
}
#[test]
fn aurora_sohftoaenc() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([370.0, 691.0], [826.0, 431.0]).len, 733.655);
}
#[test]
fn aurora_jepejbscza() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([800.0, 299.0], [96.0, 426.0]).len, 732.144);
}
#[test]
fn aurora_bnlmircdli() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([75.0, 432.0], [566.0, 697.0]).len, 719.148);
}
#[test]
fn aurora_kxheynvkot() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([906.0, 127.0], [270.0, 304.0]).len, 730.597);
}
#[test]
fn aurora_ljtiqnsxbj() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([347.0, 141.0], [195.0, 639.0]).len, 724.611);
}
#[test]
fn aurora_wklbtpzajg() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([20.0, 524.0], [717.0, 626.0]).len, 727.536);
}
#[test]
fn aurora_epohyyifai() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([190.0, 282.0], [821.0, 616.0]).len, 723.315);
}
#[test]
fn aurora_cfekwkyyvb() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([657.0, 409.0], [57.0, 538.0]).len, 722.147);
}
#[test]
fn aurora_dfjrcmdkza() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([466.0, 498.0], [734.0, 243.0]).len, 726.36);
}
#[test]
fn aurora_xnvhyypvrs() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([974.0, 400.0], [326.0, 186.0]).len, 717.958);
}
#[test]
fn aurora_nwntjytttt() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([123.0, 601.0], [827.0, 603.0]).len, 741.317);
}
#[test]
fn aurora_nweyehgzps() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([86.0, 125.0], [481.0, 565.0]).len, 731.59);
}
#[test]
fn aurora_hsbojnfque() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([790.0, 156.0], [899.0, 620.0]).len, 731.447);
}
#[test]
fn aurora_ypiapqkwtz() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([746.0, 234.0], [867.0, 619.0]).len, 733.36);
}
#[test]
fn aurora_bumkxmsker() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([143.0, 439.0], [741.0, 140.0]).len, 727.75);
}
#[test]
fn aurora_ywwuyqdslh() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([604.0, 105.0], [844.0, 557.0]).len, 729.482);
}
#[test]
fn aurora_pnbpkpdfnb() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([195.0, 575.0], [585.0, 117.0]).len, 730.345);
}
#[test]
fn aurora_nrctrnlkmb() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([732.0, 82.0], [772.0, 562.0]).len, 734.128);
}
#[test]
fn aurora_apaxlgistb() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([760.0, 702.0], [133.0, 701.0]).len, 736.195);
}
#[test]
fn aurora_kqncqbwfef() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([688.0, 219.0], [132.0, 485.0]).len, 723.859);
}
#[test]
fn aurora_luxvldnmwl() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([322.0, 66.0], [609.0, 510.0]).len, 731.034);
}
#[test]
fn aurora_rarxmknloh() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([483.0, 504.0], [719.0, 247.0]).len, 734.021);
}
#[test]
fn aurora_hmcjzybwzi() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([61.0, 658.0], [296.0, 156.0]).len, 735.773);
}
#[test]
fn aurora_lbiucgidsw() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([876.0, 572.0], [630.0, 184.0]).len, 734.392);
}
#[test]
fn aurora_cqfptpauib() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([403.0, 157.0], [824.0, 588.0]).len, 734.08);
}
#[test]
fn aurora_iwypkzaspb() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([611.0, 725.0], [483.0, 327.0]).len, 744.141);
}
#[test]
fn aurora_iubpvownbh() {
    let aurora = aurora_mesh();
    known_failure!(aurora.path([783.0, 537.0], [359.0, 212.0]).len, 734.291);
}
#[test]
fn aurora_ulmmckvedy() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([709.0, 187.0], [126.0, 478.0]).len, 735.059);
}
#[test]
fn aurora_ewyminzgfs() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([43.0, 595.0], [413.0, 151.0]).len, 733.207);
}
#[test]
fn aurora_kzwfxnwyuh() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([46.0, 649.0], [268.0, 160.0]).len, 735.498);
}
#[test]
fn aurora_nnavqtnobf() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([589.0, 417.0], [393.0, 659.0]).len, 734.069);
}
#[test]
fn aurora_vfgpyvlcvc() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([310.0, 653.0], [296.0, 168.0]).len, 738.114);
}
#[test]
fn aurora_mvtsaycajn() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([836.0, 104.0], [838.0, 627.0]).len, 738.484);
}
#[test]
fn aurora_pqsomedyfg() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([123.0, 549.0], [624.0, 439.0]).len, 737.617);
}
#[test]
fn aurora_orrzcbwhxo() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([472.0, 663.0], [78.0, 222.0]).len, 733.605);
}
#[test]
fn aurora_wdrskrreas() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([303.0, 488.0], [886.0, 258.0]).len, 732.206);
}
#[test]
fn aurora_kvwclbpsag() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([300.0, 206.0], [820.0, 503.0]).len, 737.31);
}
#[test]
fn aurora_bfmzdlucmz() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([782.0, 58.0], [997.0, 577.0]).len, 735.945);
}
#[test]
fn aurora_dthpvuucxa() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([863.0, 531.0], [722.0, 58.0]).len, 739.436);
}
#[test]
fn aurora_iomtuiehzv() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([864.0, 111.0], [927.0, 569.0]).len, 738.587);
}
#[test]
fn aurora_nvpgkulajf() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([742.0, 70.0], [505.0, 444.0]).len, 732.529);
}
#[test]
fn aurora_qaottlnprl() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([13.0, 351.0], [714.0, 346.0]).len, 739.577);
}
#[test]
fn aurora_zdfihxislt() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([24.0, 378.0], [690.0, 599.0]).len, 736.535);
}
#[test]
fn aurora_lyccbwplml() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([756.0, 498.0], [670.0, 106.0]).len, 743.014);
}
#[test]
fn aurora_xhrmoawagb() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([163.0, 125.0], [619.0, 516.0]).len, 735.239);
}
#[test]
fn aurora_xamtxwjqrp() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([874.0, 501.0], [195.0, 651.0]).len, 752.015);
}
#[test]
fn aurora_yoxqjkzbgk() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([69.0, 450.0], [656.0, 75.0]).len, 735.656);
}
#[test]
fn aurora_rxdtyojshp() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([807.0, 685.0], [130.0, 469.0]).len, 734.49);
}
#[test]
fn aurora_rixrmmnyfd() {
    let aurora = aurora_mesh();
    known_failure!(aurora.path([878.0, 525.0], [434.0, 74.0]).len, 737.183);
}
#[test]
fn aurora_lisecnqwli() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([846.0, 154.0], [120.0, 246.0]).len, 764.437);
}
#[test]
fn aurora_uiitfglnwh() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([524.0, 269.0], [584.0, 720.0]).len, 746.733);
}
#[test]
fn aurora_fbyudbujis() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([61.0, 569.0], [591.0, 395.0]).len, 743.949);
}
#[test]
fn aurora_jgujbdpnsl() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([142.0, 714.0], [864.0, 634.0]).len, 751.243);
}
#[test]
fn aurora_jngxgdecjh() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([567.0, 662.0], [351.0, 159.0]).len, 748.816);
}
#[test]
fn aurora_efjdcsshsf() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([121.0, 122.0], [823.0, 246.0]).len, 739.884);
}
#[test]
fn aurora_sjawjccbay() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([97.0, 144.0], [615.0, 435.0]).len, 747.803);
}
#[test]
fn aurora_hvngxonpkt() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([162.0, 207.0], [717.0, 567.0]).len, 735.83);
}
#[test]
fn aurora_nmbbkfnagu() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([703.0, 142.0], [928.0, 582.0]).len, 742.786);
}
#[test]
fn aurora_lxnrnhuqln() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([910.0, 522.0], [393.0, 89.0]).len, 741.367);
}
#[test]
fn aurora_myfpyatrza() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([749.0, 234.0], [63.0, 325.0]).len, 743.953);
}
#[test]
fn aurora_fllfvfgrmv() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([747.0, 424.0], [35.0, 538.0]).len, 740.238);
}
#[test]
fn aurora_nyhmlfkxsb() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([293.0, 647.0], [547.0, 153.0]).len, 748.603);
}
#[test]
fn aurora_gxbesossom() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([290.0, 148.0], [984.0, 366.0]).len, 742.861);
}
#[test]
fn aurora_obmsilmhis() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([935.0, 630.0], [219.0, 691.0]).len, 751.093);
}
#[test]
fn aurora_ptwwqypinu() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([863.0, 432.0], [177.0, 314.0]).len, 739.758);
}
#[test]
fn aurora_pmlotncczl() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([38.0, 632.0], [748.0, 716.0]).len, 759.416);
}
#[test]
fn aurora_yzoeuxauvp() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([289.0, 151.0], [933.0, 474.0]).len, 743.734);
}
#[test]
fn aurora_sxkcpuzymv() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([343.0, 179.0], [688.0, 590.0]).len, 743.15);
}
#[test]
fn aurora_lvceaiuiqm() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([57.0, 634.0], [414.0, 165.0]).len, 748.88);
}
#[test]
fn aurora_jtsmufaksv() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([751.0, 355.0], [134.0, 521.0]).len, 746.126);
}
#[test]
fn aurora_rxajrqxyci() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([96.0, 481.0], [787.0, 347.0]).len, 760.282);
}
#[test]
fn aurora_kbzpjylkyl() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([816.0, 491.0], [88.0, 482.0]).len, 753.376);
}
#[test]
fn aurora_kcnviuokpv() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([823.0, 161.0], [518.0, 617.0]).len, 752.396);
}
#[test]
fn aurora_zvhpsrkzns() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([1.0, 657.0], [207.0, 118.0]).len, 754.04);
}
#[test]
fn aurora_zineixfqil() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([63.0, 665.0], [313.0, 137.0]).len, 756.231);
}
#[test]
fn aurora_vanswcculg() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([666.0, 85.0], [39.0, 427.0]).len, 750.919);
}
#[test]
fn aurora_pxdinzesds() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([279.0, 641.0], [766.0, 396.0]).len, 755.911);
}
#[test]
fn aurora_yyybkjujoz() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([561.0, 636.0], [812.0, 144.0]).len, 752.731);
}
#[test]
fn aurora_nspuqybmts() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([833.0, 493.0], [168.0, 454.0]).len, 747.262);
}
#[test]
fn aurora_pzknlnzmlk() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([674.0, 379.0], [77.0, 551.0]).len, 750.534);
}
#[test]
fn aurora_ywsuupbwhf() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([856.0, 459.0], [119.0, 546.0]).len, 762.504);
}
#[test]
fn aurora_rcregfqmmm() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([935.0, 598.0], [214.0, 469.0]).len, 753.018);
}
#[test]
fn aurora_fuehyczvwq() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([669.0, 522.0], [153.0, 174.0]).len, 746.721);
}
#[test]
fn aurora_crwiqfjfpj() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([201.0, 344.0], [952.0, 306.0]).len, 764.864);
}
#[test]
fn aurora_upvjjtvhnm() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([61.0, 163.0], [654.0, 460.0]).len, 746.031);
}
#[test]
fn aurora_fjzcokftpe() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([442.0, 694.0], [923.0, 267.0]).len, 757.996);
}
#[test]
fn aurora_thdynyfpyt() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([448.0, 151.0], [262.0, 637.0]).len, 755.258);
}
#[test]
fn aurora_iqpskbmald() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([849.0, 513.0], [115.0, 580.0]).len, 758.004);
}
#[test]
fn aurora_eyfmljdyem() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([400.0, 226.0], [37.0, 675.0]).len, 760.259);
}
#[test]
fn aurora_yqiloyvjed() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([259.0, 184.0], [324.0, 688.0]).len, 757.819);
}
#[test]
fn aurora_hmgpjgqyfc() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([88.0, 427.0], [721.0, 128.0]).len, 754.697);
}
#[test]
fn aurora_qmqjbdbtnk() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([692.0, 615.0], [850.0, 132.0]).len, 764.572);
}
#[test]
fn aurora_bseccvhwza() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([171.0, 664.0], [638.0, 293.0]).len, 761.036);
}
#[test]
fn aurora_wwqebbogmo() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([869.0, 345.0], [154.0, 206.0]).len, 770.079);
}
#[test]
fn aurora_mvstpynvam() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([854.0, 155.0], [465.0, 429.0]).len, 756.429);
}
#[test]
fn aurora_ndgqnjlekh() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([45.0, 187.0], [796.0, 193.0]).len, 776.824);
}
#[test]
fn aurora_nplqlmgshc() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([834.0, 712.0], [139.0, 569.0]).len, 752.973);
}
#[test]
fn aurora_ugpkjtjgco() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([209.0, 76.0], [500.0, 585.0]).len, 764.763);
}
#[test]
fn aurora_xlgtkmiszf() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([167.0, 290.0], [873.0, 183.0]).len, 763.988);
}
#[test]
fn aurora_rmkqbgsdhu() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([359.0, 282.0], [812.0, 554.0]).len, 757.016);
}
#[test]
fn aurora_obaxjxuwgq() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([361.0, 145.0], [170.0, 681.0]).len, 768.188);
}
#[test]
fn aurora_rvuphyojjr() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([915.0, 288.0], [182.0, 162.0]).len, 763.705);
}
#[test]
fn aurora_lzyydojdlf() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([743.0, 162.0], [909.0, 640.0]).len, 762.428);
}
#[test]
fn aurora_kqgmyqboqy() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([568.0, 663.0], [558.0, 190.0]).len, 765.991);
}
#[test]
fn aurora_rwvrxaofgf() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([411.0, 637.0], [991.0, 301.0]).len, 769.401);
}
#[test]
fn aurora_cwfemenwrq() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([859.0, 258.0], [116.0, 282.0]).len, 769.241);
}
#[test]
fn aurora_xuwutsssng() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([632.0, 216.0], [845.0, 638.0]).len, 771.128);
}
#[test]
fn aurora_hozlupkkcb() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([410.0, 190.0], [833.0, 636.0]).len, 762.815);
}
#[test]
fn aurora_wwhcrzjnkz() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([627.0, 588.0], [365.0, 165.0]).len, 758.877);
}
#[test]
fn aurora_zeiwacpown() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([862.0, 418.0], [151.0, 284.0]).len, 756.604);
}
#[test]
fn aurora_xvdvvwyjvn() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([368.0, 494.0], [765.0, 67.0]).len, 764.23);
}
#[test]
fn aurora_evbiboargw() {
    let aurora = aurora_mesh();
    known_failure!(aurora.path([882.0, 532.0], [366.0, 262.0]).len, 763.192);
}
#[test]
fn aurora_xqyvlznfar() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([142.0, 501.0], [753.0, 162.0]).len, 780.337);
}
#[test]
fn aurora_dxsvtvkpeb() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([494.0, 508.0], [740.0, 82.0]).len, 762.84);
}
#[test]
fn aurora_yshfkvcozt() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([378.0, 694.0], [322.0, 201.0]).len, 771.95);
}
#[test]
fn aurora_wrucxftaik() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([406.0, 610.0], [458.0, 67.0]).len, 776.348);
}
#[test]
fn aurora_jlzflcaqte() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([696.0, 76.0], [815.0, 597.0]).len, 774.397);
}
#[test]
fn aurora_trbhrehech() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([417.0, 239.0], [547.0, 682.0]).len, 770.817);
}
#[test]
fn aurora_iuontdxlaa() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([280.0, 575.0], [277.0, 30.0]).len, 769.487);
}
#[test]
fn aurora_qcksecmgat() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([201.0, 377.0], [838.0, 702.0]).len, 765.708);
}
#[test]
fn aurora_enhabpvyve() {
    let aurora = aurora_mesh();
    known_failure!(aurora.path([773.0, 655.0], [522.0, 155.0]).len, 768.509);
}
#[test]
fn aurora_pimwmldpuc() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([801.0, 433.0], [68.0, 370.0]).len, 771.243);
}
#[test]
fn aurora_zaszdiimqo() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([907.0, 538.0], [164.0, 511.0]).len, 769.993);
}
#[test]
fn aurora_zgnhbxiihn() {
    let aurora = aurora_mesh();
    known_failure!(aurora.path([779.0, 624.0], [585.0, 207.0]).len, 768.016);
}
#[test]
fn aurora_xfudkrktfu() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([289.0, 746.0], [704.0, 279.0]).len, 780.097);
}
#[test]
fn aurora_jxnhaizmgd() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([465.0, 584.0], [647.0, 71.0]).len, 776.097);
}
#[test]
fn aurora_qfcdmomnhv() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([223.0, 397.0], [955.0, 367.0]).len, 785.554);
}
#[test]
fn aurora_cekusjjkbt() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([726.0, 187.0], [473.0, 592.0]).len, 770.797);
}
#[test]
fn aurora_rvedekxkov() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([697.0, 318.0], [252.0, 636.0]).len, 784.393);
}
#[test]
fn aurora_gyvkopiwyu() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([224.0, 120.0], [100.0, 694.0]).len, 771.919);
}
#[test]
fn aurora_qqaqxmylwk() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([466.0, 632.0], [140.0, 130.0]).len, 777.416);
}
#[test]
fn aurora_eliyvvvmjg() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([237.0, 140.0], [361.0, 671.0]).len, 780.241);
}
#[test]
fn aurora_bqidjhskmf() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([970.0, 348.0], [359.0, 741.0]).len, 776.86);
}
#[test]
fn aurora_ruyyghlysi() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([150.0, 263.0], [905.0, 331.0]).len, 783.426);
}
#[test]
fn aurora_tgpvbpjtei() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([856.0, 680.0], [187.0, 354.0]).len, 774.501);
}
#[test]
fn aurora_dhintbrnld() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([116.0, 422.0], [733.0, 267.0]).len, 763.463);
}
#[test]
fn aurora_mholohngcc() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([334.0, 224.0], [918.0, 529.0]).len, 773.184);
}
#[test]
fn aurora_culdosbgrj() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([906.0, 316.0], [276.0, 553.0]).len, 772.252);
}
#[test]
fn aurora_sgwxwuclva() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([15.0, 477.0], [651.0, 695.0]).len, 775.985);
}
#[test]
fn aurora_cwtvdgbstj() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([431.0, 606.0], [101.0, 114.0]).len, 779.39);
}
#[test]
fn aurora_tedfgdboqb() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([169.0, 296.0], [876.0, 584.0]).len, 772.223);
}
#[test]
fn aurora_hlgvonvhhp() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([630.0, 103.0], [611.0, 589.0]).len, 773.523);
}
#[test]
fn aurora_zyosimufob() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([65.0, 494.0], [757.0, 371.0]).len, 773.633);
}
#[test]
fn aurora_pngvjcxrex() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([125.0, 134.0], [590.0, 596.0]).len, 784.664);
}
#[test]
fn aurora_avwaenztha() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([563.0, 553.0], [927.0, 130.0]).len, 778.087);
}
#[test]
fn aurora_njjuzjguzq() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([109.0, 510.0], [849.0, 689.0]).len, 773.372);
}
#[test]
fn aurora_njzxpeuafl() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([193.0, 536.0], [674.0, 129.0]).len, 773.548);
}
#[test]
fn aurora_usltobsvyu() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([212.0, 691.0], [606.0, 407.0]).len, 783.267);
}
#[test]
fn aurora_dyzuatdcfv() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([27.0, 391.0], [749.0, 499.0]).len, 775.272);
}
#[test]
fn aurora_iosavpwewd() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([89.0, 505.0], [783.0, 703.0]).len, 773.531);
}
#[test]
fn aurora_reefxwerui() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([163.0, 703.0], [586.0, 420.0]).len, 778.91);
}
#[test]
fn aurora_aehtqzblmp() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([726.0, 686.0], [811.0, 179.0]).len, 786.745);
}
#[test]
fn aurora_swzcizbmjp() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([336.0, 665.0], [608.0, 406.0]).len, 782.168);
}
#[test]
fn aurora_njklfnlias() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([937.0, 644.0], [207.0, 460.0]).len, 781.285);
}
#[test]
fn aurora_vfjgkrktzx() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([636.0, 53.0], [310.0, 574.0]).len, 782.684);
}
#[test]
fn aurora_ybzlwjjlqp() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([764.0, 235.0], [301.0, 476.0]).len, 777.682);
}
#[test]
fn aurora_stssytinkn() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([571.0, 549.0], [195.0, 74.0]).len, 784.066);
}
#[test]
fn aurora_trkgtvzbaw() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([126.0, 228.0], [828.0, 478.0]).len, 777.353);
}
#[test]
fn aurora_tnlhkasczn() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([248.0, 557.0], [912.0, 425.0]).len, 783.548);
}
#[test]
fn aurora_sdnhhkwuyx() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([502.0, 139.0], [354.0, 641.0]).len, 782.584);
}
#[test]
fn aurora_jmkgaoyney() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([829.0, 100.0], [523.0, 586.0]).len, 786.654);
}
#[test]
fn aurora_ngrvvqsvyx() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([643.0, 136.0], [508.0, 636.0]).len, 787.74);
}
#[test]
fn aurora_wivxxxdegg() {
    let aurora = aurora_mesh();
    known_failure!(aurora.path([732.0, 607.0], [394.0, 197.0]).len, 785.399);
}
#[test]
fn aurora_umerqjgpaz() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([163.0, 388.0], [878.0, 494.0]).len, 778.183);
}
#[test]
fn aurora_eihsnulbpt() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([190.0, 69.0], [156.0, 647.0]).len, 785.474);
}
#[test]
fn aurora_ezltgzmvfk() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([742.0, 318.0], [101.0, 509.0]).len, 785.202);
}
#[test]
fn aurora_rtluzvriqa() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([61.0, 164.0], [703.0, 414.0]).len, 776.055);
}
#[test]
fn aurora_kicvzapgns() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([266.0, 210.0], [763.0, 496.0]).len, 777.107);
}
#[test]
fn aurora_wubangtdkn() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([389.0, 704.0], [303.0, 307.0]).len, 796.234);
}
#[test]
fn aurora_umnbcaxzld() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([56.0, 477.0], [819.0, 446.0]).len, 786.195);
}
#[test]
fn aurora_tqeemnrlvi() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([860.0, 107.0], [472.0, 459.0]).len, 787.235);
}
#[test]
fn aurora_hcitjmzqmy() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([389.0, 478.0], [830.0, 108.0]).len, 791.234);
}
#[test]
fn aurora_ttsjqsqolb() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([462.0, 175.0], [824.0, 716.0]).len, 793.267);
}
#[test]
fn aurora_ejdnlxyvcr() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([322.0, 51.0], [599.0, 592.0]).len, 794.362);
}
#[test]
fn aurora_wspjyiupic() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([554.0, 594.0], [751.0, 107.0]).len, 791.632);
}
#[test]
fn aurora_jtoooqgvby() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([739.0, 67.0], [39.0, 372.0]).len, 784.99);
}
#[test]
fn aurora_ihiktaowlp() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([186.0, 575.0], [816.0, 367.0]).len, 790.847);
}
#[test]
fn aurora_umtvjedanl() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([267.0, 17.0], [967.0, 315.0]).len, 784.347);
}
#[test]
fn aurora_lmegoviydo() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([598.0, 160.0], [900.0, 616.0]).len, 788.498);
}
#[test]
fn aurora_flwbnlmwfv() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([745.0, 178.0], [653.0, 651.0]).len, 790.096);
}
#[test]
fn aurora_mzigqfmepu() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([44.0, 685.0], [247.0, 85.0]).len, 798.323);
}
#[test]
fn aurora_vkgzzllqcr() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([241.0, 629.0], [601.0, 146.0]).len, 793.24);
}
#[test]
fn aurora_vfwcpjnvda() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([842.0, 176.0], [88.0, 318.0]).len, 806.381);
}
#[test]
fn aurora_denbbqpwff() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([479.0, 239.0], [591.0, 726.0]).len, 796.617);
}
#[test]
fn aurora_zujbupmetg() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([24.0, 456.0], [724.0, 561.0]).len, 789.647);
}
#[test]
fn aurora_zbyaiohufx() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([318.0, 156.0], [403.0, 631.0]).len, 791.8);
}
#[test]
fn aurora_tftaplrboc() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([655.0, 135.0], [788.0, 625.0]).len, 788.659);
}
#[test]
fn aurora_tlmxkadyng() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([738.0, 52.0], [827.0, 609.0]).len, 794.599);
}
#[test]
fn aurora_sxlrrlvvzm() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([703.0, 104.0], [67.0, 475.0]).len, 784.015);
}
#[test]
fn aurora_cewyacrfhz() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([910.0, 259.0], [136.0, 178.0]).len, 799.261);
}
#[test]
fn aurora_sovysztkcg() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([81.0, 398.0], [728.0, 53.0]).len, 791.277);
}
#[test]
fn aurora_pgbckyoaxl() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([226.0, 473.0], [953.0, 394.0]).len, 793.963);
}
#[test]
fn aurora_hkslfkuzow() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([524.0, 630.0], [710.0, 146.0]).len, 797.845);
}
#[test]
fn aurora_pykohahdcn() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([635.0, 605.0], [418.0, 244.0]).len, 787.687);
}
#[test]
fn aurora_wopfsvfsci() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([128.0, 450.0], [785.0, 183.0]).len, 798.318);
}
#[test]
fn aurora_okcjvvalqm() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([921.0, 421.0], [164.0, 353.0]).len, 800.321);
}
#[test]
fn aurora_bjoqarnnbw() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([713.0, 671.0], [676.0, 250.0]).len, 797.463);
}
#[test]
fn aurora_dzodgounku() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([391.0, 193.0], [478.0, 673.0]).len, 804.09);
}
#[test]
fn aurora_ovcdtqytyb() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([1005.0, 595.0], [701.0, 154.0]).len, 792.776);
}
#[test]
fn aurora_tbbwosognb() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([120.0, 150.0], [547.0, 600.0]).len, 801.565);
}
#[test]
fn aurora_bkolfwdxyg() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([109.0, 326.0], [844.0, 440.0]).len, 796.331);
}
#[test]
fn aurora_ixgrgcxqpb() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([709.0, 629.0], [832.0, 84.0]).len, 803.879);
}
#[test]
fn aurora_iblgvvubyx() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([200.0, 180.0], [453.0, 681.0]).len, 798.43);
}
#[test]
fn aurora_jqnmjqgjoz() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([101.0, 165.0], [894.0, 242.0]).len, 811.021);
}
#[test]
fn aurora_cukftsqogd() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([509.0, 215.0], [340.0, 669.0]).len, 799.511);
}
#[test]
fn aurora_aopcieneqx() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([835.0, 318.0], [60.0, 393.0]).len, 803.981);
}
#[test]
fn aurora_ogeqmgmanj() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([73.0, 623.0], [322.0, 46.0]).len, 807.462);
}
#[test]
fn aurora_zudrnkfbhx() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([215.0, 591.0], [990.0, 616.0]).len, 808.599);
}
#[test]
fn aurora_bmojpkqlqa() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([316.0, 85.0], [28.0, 631.0]).len, 804.592);
}
#[test]
fn aurora_xwkpkxpbwk() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([180.0, 413.0], [905.0, 471.0]).len, 795.09);
}
#[test]
fn aurora_djgalrwjjv() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([103.0, 443.0], [751.0, 68.0]).len, 795.031);
}
#[test]
fn aurora_zxzgqcwhtr() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([148.0, 385.0], [844.0, 666.0]).len, 800.41);
}
#[test]
fn aurora_qwyugqxcdp() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([884.0, 153.0], [106.0, 311.0]).len, 822.316);
}
#[test]
fn aurora_hbyxxnxdwv() {
    let aurora = aurora_mesh();
    known_failure!(aurora.path([732.0, 521.0], [488.0, 51.0]).len, 801.101);
}
#[test]
fn aurora_wufuugkfzr() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([172.0, 652.0], [596.0, 127.0]).len, 803.854);
}
#[test]
fn aurora_gikmehczfe() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([489.0, 209.0], [264.0, 732.0]).len, 806.518);
}
#[test]
fn aurora_ljsnqnuipc() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([330.0, 273.0], [671.0, 717.0]).len, 809.472);
}
#[test]
fn aurora_wtlcdpbcmm() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([771.0, 426.0], [33.0, 664.0]).len, 801.109);
}
#[test]
fn aurora_rqqoenygqq() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([419.0, 222.0], [856.0, 548.0]).len, 804.135);
}
#[test]
fn aurora_jhqhxlepjp() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([312.0, 673.0], [446.0, 158.0]).len, 808.4);
}
#[test]
fn aurora_fwdjotfgnt() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([276.0, 614.0], [367.0, 87.0]).len, 808.781);
}
#[test]
fn aurora_hqrphslqfc() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([222.0, 65.0], [356.0, 621.0]).len, 805.102);
}
#[test]
fn aurora_bbgxxexkcf() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([154.0, 555.0], [794.0, 296.0]).len, 801.113);
}
#[test]
fn aurora_snlasrrigg() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([781.0, 216.0], [349.0, 595.0]).len, 806.74);
}
#[test]
fn aurora_ntueekpxto() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([292.0, 674.0], [234.0, 62.0]).len, 807.282);
}
#[test]
fn aurora_aqhfgsblrw() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([642.0, 714.0], [807.0, 176.0]).len, 812.637);
}
#[test]
fn aurora_qyqppwavew() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([190.0, 523.0], [863.0, 385.0]).len, 812.721);
}
#[test]
fn aurora_dugtthssnt() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([261.0, 316.0], [1006.0, 590.0]).len, 804.976);
}
#[test]
fn aurora_jujxliscow() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([174.0, 241.0], [411.0, 695.0]).len, 805.028);
}
#[test]
fn aurora_weehxbvumy() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([800.0, 138.0], [166.0, 499.0]).len, 811.926);
}
#[test]
fn aurora_pndjlccukm() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([569.0, 656.0], [612.0, 197.0]).len, 808.309);
}
#[test]
fn aurora_rwrlpaogtu() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([791.0, 380.0], [318.0, 696.0]).len, 810.652);
}
#[test]
fn aurora_mbcvghbdpw() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([169.0, 188.0], [690.0, 606.0]).len, 805.863);
}
#[test]
fn aurora_rpnyghcyhs() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([192.0, 516.0], [779.0, 126.0]).len, 809.786);
}
#[test]
fn aurora_wnsnmyvzcm() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([807.0, 726.0], [118.0, 396.0]).len, 808.414);
}
#[test]
fn aurora_ohrxhecasy() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([114.0, 422.0], [890.0, 245.0]).len, 816.325);
}
#[test]
fn aurora_hpwyjzyyas() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([402.0, 171.0], [940.0, 614.0]).len, 809.104);
}
#[test]
fn aurora_punvxcyoov() {
    let aurora = aurora_mesh();
    known_failure!(aurora.path([853.0, 511.0], [293.0, 84.0]).len, 807.84);
}
#[test]
fn aurora_ftmtcdeyar() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([227.0, 155.0], [315.0, 721.0]).len, 816.37);
}
#[test]
fn aurora_dxdxmjikfv() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([24.0, 654.0], [585.0, 387.0]).len, 809.841);
}
#[test]
fn aurora_drpcodubkj() {
    let aurora = aurora_mesh();
    known_failure!(aurora.path([899.0, 555.0], [479.0, 52.0]).len, 817.004);
}
#[test]
fn aurora_uodhamzyow() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([767.0, 526.0], [168.0, 238.0]).len, 809.545);
}
#[test]
fn aurora_eqdqbtxytm() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([690.0, 110.0], [369.0, 590.0]).len, 814.191);
}
#[test]
fn aurora_jpvdgriurt() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([333.0, 541.0], [743.0, 251.0]).len, 812.785);
}
#[test]
fn aurora_xpvrftszoc() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([643.0, 131.0], [828.0, 657.0]).len, 816.101);
}
#[test]
fn aurora_flpjyswcax() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([338.0, 746.0], [1019.0, 428.0]).len, 813.889);
}
#[test]
fn aurora_haciwikjnc() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([116.0, 434.0], [872.0, 389.0]).len, 815.274);
}
#[test]
fn aurora_cjhxlpsnso() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([694.0, 690.0], [552.0, 146.0]).len, 817.379);
}
#[test]
fn aurora_rrbslpvzhc() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([2.0, 582.0], [614.0, 200.0]).len, 811.781);
}
#[test]
fn aurora_mgijeejosz() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([863.0, 622.0], [86.0, 468.0]).len, 817.818);
}
#[test]
fn aurora_fgiqwqlvkq() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([965.0, 567.0], [201.0, 357.0]).len, 818.133);
}
#[test]
fn aurora_nuobvnuxdq() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([160.0, 584.0], [828.0, 344.0]).len, 815.89);
}
#[test]
fn aurora_wdwbwxssls() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([95.0, 500.0], [898.0, 577.0]).len, 824.9);
}
#[test]
fn aurora_joqmbqspxa() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([336.0, 516.0], [910.0, 170.0]).len, 815.916);
}
#[test]
fn aurora_qxwksggroz() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([183.0, 417.0], [972.0, 284.0]).len, 827.715);
}
#[test]
fn aurora_lvkbyssbmy() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([310.0, 589.0], [442.0, 64.0]).len, 819.289);
}
#[test]
fn aurora_argpytfaql() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([448.0, 201.0], [663.0, 673.0]).len, 824.22);
}
#[test]
fn aurora_qyhrwxkkhl() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([682.0, 626.0], [220.0, 198.0]).len, 812.117);
}
#[test]
fn aurora_eargojsxav() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([362.0, 239.0], [963.0, 549.0]).len, 815.571);
}
#[test]
fn aurora_btmkqysign() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([27.0, 175.0], [833.0, 200.0]).len, 830.694);
}
#[test]
fn aurora_nrjzgvfpoe() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([840.0, 235.0], [240.0, 524.0]).len, 819.65);
}
#[test]
fn aurora_yuetfabqsn() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([627.0, 147.0], [232.0, 634.0]).len, 825.337);
}
#[test]
fn aurora_hlfydfwfkt() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([584.0, 174.0], [791.0, 688.0]).len, 824.525);
}
#[test]
fn aurora_etewnvnjha() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([598.0, 609.0], [117.0, 176.0]).len, 826.125);
}
#[test]
fn aurora_fjfesseatu() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([731.0, 61.0], [569.0, 593.0]).len, 827.645);
}
#[test]
fn aurora_tzgkukbtta() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([810.0, 577.0], [11.0, 554.0]).len, 831.355);
}
#[test]
fn aurora_xzxxicvdfi() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([435.0, 106.0], [511.0, 652.0]).len, 834.216);
}
#[test]
fn aurora_yopwpxvnhb() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([806.0, 700.0], [327.0, 268.0]).len, 822.886);
}
#[test]
fn aurora_naidinrdoh() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([111.0, 272.0], [781.0, 678.0]).len, 822.422);
}
#[test]
fn aurora_fbrhnzfpxf() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([459.0, 55.0], [561.0, 617.0]).len, 824.329);
}
#[test]
fn aurora_bkmnswynhb() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([766.0, 563.0], [225.0, 235.0]).len, 821.762);
}
#[test]
fn aurora_tgwfxobvzg() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([765.0, 566.0], [58.0, 445.0]).len, 822.746);
}
#[test]
fn aurora_ykahssntei() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([218.0, 494.0], [973.0, 374.0]).len, 830.965);
}
#[test]
fn aurora_jxhfqtyouz() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([258.0, 395.0], [991.0, 434.0]).len, 826.694);
}
#[test]
fn aurora_vnfwmxdwjk() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([4.0, 563.0], [640.0, 122.0]).len, 824.564);
}
#[test]
fn aurora_opivlvxlwu() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([328.0, 108.0], [712.0, 564.0]).len, 824.613);
}
#[test]
fn aurora_gaavlizdmk() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([871.0, 560.0], [68.0, 548.0]).len, 834.2);
}
#[test]
fn aurora_mtmjlasgtw() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([990.0, 416.0], [235.0, 479.0]).len, 826.094);
}
#[test]
fn aurora_lwqvilfxmt() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([673.0, 74.0], [200.0, 589.0]).len, 827.608);
}
#[test]
fn aurora_xwlwswnjui() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([445.0, 183.0], [998.0, 631.0]).len, 822.71);
}
#[test]
fn aurora_kjkuhsxwuv() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([815.0, 411.0], [37.0, 381.0]).len, 826.113);
}
#[test]
fn aurora_wxiibnrldy() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([851.0, 368.0], [275.0, 618.0]).len, 835.956);
}
#[test]
fn aurora_fjuqgfofyx() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([184.0, 671.0], [951.0, 467.0]).len, 840.312);
}
#[test]
fn aurora_zepupfxnys() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([671.0, 245.0], [458.0, 630.0]).len, 833.314);
}
#[test]
fn aurora_giqzqjulsg() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([650.0, 142.0], [550.0, 659.0]).len, 837.833);
}
#[test]
fn aurora_wtylnyrsxm() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([845.0, 234.0], [347.0, 660.0]).len, 835.072);
}
#[test]
fn aurora_qeweovntgg() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([16.0, 406.0], [777.0, 647.0]).len, 824.615);
}
#[test]
fn aurora_epubrprban() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([139.0, 596.0], [919.0, 530.0]).len, 834.986);
}
#[test]
fn aurora_dsseyxfkya() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([130.0, 102.0], [205.0, 683.0]).len, 836.915);
}
#[test]
fn aurora_qbkbyavmzj() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([135.0, 429.0], [941.0, 292.0]).len, 841.851);
}
#[test]
fn aurora_sldjbrpokw() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([44.0, 666.0], [649.0, 338.0]).len, 848.395);
}
#[test]
fn aurora_wlafkwbvjk() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([892.0, 121.0], [593.0, 655.0]).len, 836.481);
}
#[test]
fn aurora_ezrozgsnts() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([891.0, 289.0], [333.0, 714.0]).len, 835.262);
}
#[test]
fn aurora_ljkjkmsacb() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([471.0, 637.0], [856.0, 135.0]).len, 838.911);
}
#[test]
fn aurora_spjervnjon() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([730.0, 261.0], [288.0, 555.0]).len, 835.407);
}
#[test]
fn aurora_oteztbocbs() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([602.0, 656.0], [185.0, 123.0]).len, 840.842);
}
#[test]
fn aurora_aulhxuatlq() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([742.0, 94.0], [502.0, 582.0]).len, 834.949);
}
#[test]
fn aurora_iycjazqhyw() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([775.0, 83.0], [371.0, 513.0]).len, 833.295);
}
#[test]
fn aurora_chptfdylfy() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([97.0, 390.0], [855.0, 136.0]).len, 849.319);
}
#[test]
fn aurora_rbpqslkcea() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([793.0, 723.0], [15.0, 514.0]).len, 833.775);
}
#[test]
fn aurora_bfqewblpkx() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([12.0, 649.0], [623.0, 283.0]).len, 840.518);
}
#[test]
fn aurora_qfbrunjyup() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([388.0, 113.0], [851.0, 646.0]).len, 835.125);
}
#[test]
fn aurora_jcfnsdhpxj() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([704.0, 293.0], [152.0, 672.0]).len, 845.296);
}
#[test]
fn aurora_grewhumdbr() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([153.0, 353.0], [926.0, 611.0]).len, 838.012);
}
#[test]
fn aurora_fpgogfcffu() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([616.0, 173.0], [508.0, 659.0]).len, 846.701);
}
#[test]
fn aurora_ceetlimqht() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([914.0, 478.0], [136.0, 724.0]).len, 849.666);
}
#[test]
fn aurora_bcxznhowjj() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([186.0, 491.0], [824.0, 209.0]).len, 837.497);
}
#[test]
fn aurora_fqpypeuqxi() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([731.0, 249.0], [475.0, 638.0]).len, 835.744);
}
#[test]
fn aurora_igxkkiiemz() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([252.0, 590.0], [684.0, 243.0]).len, 835.521);
}
#[test]
fn aurora_ncojylcony() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([21.0, 534.0], [782.0, 343.0]).len, 846.53);
}
#[test]
fn aurora_rgxabzbtkb() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([160.0, 528.0], [716.0, 151.0]).len, 842.423);
}
#[test]
fn aurora_kspdmgnkst() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([48.0, 433.0], [737.0, 46.0]).len, 839.675);
}
#[test]
fn aurora_rnbklwczoy() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([217.0, 91.0], [951.0, 464.0]).len, 836.113);
}
#[test]
fn aurora_hlhzginorw() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([824.0, 264.0], [370.0, 666.0]).len, 847.26);
}
#[test]
fn aurora_tjthmexswj() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([536.0, 707.0], [366.0, 157.0]).len, 847.501);
}
#[test]
fn aurora_yorauvvjlu() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([85.0, 455.0], [876.0, 244.0]).len, 845.899);
}
#[test]
fn aurora_tqodgsewzi() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([126.0, 702.0], [436.0, 146.0]).len, 844.3);
}
#[test]
fn aurora_jukmmnilnl() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([208.0, 679.0], [788.0, 350.0]).len, 843.456);
}
#[test]
fn aurora_vyxwlgcyxf() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([767.0, 260.0], [13.0, 386.0]).len, 840.46);
}
#[test]
fn aurora_vsjucqeqrf() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([585.0, 148.0], [274.0, 697.0]).len, 840.362);
}
#[test]
fn aurora_lbmspiumuv() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([111.0, 657.0], [263.0, 24.0]).len, 849.159);
}
#[test]
fn aurora_ukzqlwtarq() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([595.0, 681.0], [840.0, 165.0]).len, 846.135);
}
#[test]
fn aurora_bzpudnrbyc() {
    let aurora = aurora_mesh();
    known_failure!(aurora.path([831.0, 688.0], [610.0, 118.0]).len, 846.725);
}
#[test]
fn aurora_ndxmeyzykl() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([863.0, 220.0], [340.0, 713.0]).len, 850.467);
}
#[test]
fn aurora_aaqtarocjq() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([910.0, 623.0], [164.0, 419.0]).len, 838.546);
}
#[test]
fn aurora_aohwwzladw() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([37.0, 473.0], [849.0, 618.0]).len, 851.089);
}
#[test]
fn aurora_expnmmcqje() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([320.0, 52.0], [706.0, 536.0]).len, 841.934);
}
#[test]
fn aurora_cjvrqhfczy() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([839.0, 273.0], [272.0, 626.0]).len, 848.35);
}
#[test]
fn aurora_gzhyleliwq() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([125.0, 104.0], [527.0, 617.0]).len, 850.307);
}
#[test]
fn aurora_shkwhhyyev() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([26.0, 197.0], [883.0, 143.0]).len, 872.744);
}
#[test]
fn aurora_tgsgwkiarc() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([202.0, 701.0], [941.0, 405.0]).len, 855.231);
}
#[test]
fn aurora_hdxwgqchbv() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([396.0, 662.0], [787.0, 258.0]).len, 852.654);
}
#[test]
fn aurora_vksozdhgyp() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([1001.0, 322.0], [206.0, 106.0]).len, 844.661);
}
#[test]
fn aurora_regjvehrnl() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([561.0, 652.0], [268.0, 50.0]).len, 853.912);
}
#[test]
fn aurora_vrlfjzhpkf() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([600.0, 152.0], [780.0, 686.0]).len, 849.856);
}
#[test]
fn aurora_ekkythuijx() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([179.0, 357.0], [962.0, 599.0]).len, 846.456);
}
#[test]
fn aurora_hzcqgymtfr() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([622.0, 255.0], [136.0, 735.0]).len, 846.78);
}
#[test]
fn aurora_eyyjeauteu() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([851.0, 669.0], [634.0, 117.0]).len, 848.382);
}
#[test]
fn aurora_ackfxixlbs() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([303.0, 50.0], [912.0, 530.0]).len, 843.874);
}
#[test]
fn aurora_skmkgdvisf() {
    let aurora = aurora_mesh();
    known_failure!(aurora.path([926.0, 591.0], [376.0, 241.0]).len, 847.603);
}
#[test]
fn aurora_pfvvloslpv() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([872.0, 559.0], [101.0, 226.0]).len, 848.502);
}
#[test]
fn aurora_yuaqojsbag() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([380.0, 79.0], [648.0, 633.0]).len, 853.052);
}
#[test]
fn aurora_fvfdsgcpwv() {
    let aurora = aurora_mesh();
    known_failure!(aurora.path([982.0, 564.0], [213.0, 345.0]).len, 848.372);
}
#[test]
fn aurora_afldheqmfc() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([35.0, 320.0], [871.0, 318.0]).len, 859.241);
}
#[test]
fn aurora_ulowbpobqz() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([487.0, 47.0], [897.0, 590.0]).len, 855.591);
}
#[test]
fn aurora_bhfgjiqhoj() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([967.0, 360.0], [165.0, 133.0]).len, 850.887);
}
#[test]
fn aurora_tllnjqdwhd() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([445.0, 626.0], [831.0, 122.0]).len, 852.745);
}
#[test]
fn aurora_qecsborhkv() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([217.0, 621.0], [651.0, 112.0]).len, 852.612);
}
#[test]
fn aurora_vjsrvnhyra() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([312.0, 510.0], [810.0, 103.0]).len, 854.266);
}
#[test]
fn aurora_yufyvtwcja() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([82.0, 365.0], [894.0, 423.0]).len, 857.783);
}
#[test]
fn aurora_pjezszeakb() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([915.0, 280.0], [69.0, 324.0]).len, 865.32);
}
#[test]
fn aurora_fhxionvuij() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([877.0, 262.0], [259.0, 575.0]).len, 852.767);
}
#[test]
fn aurora_nkykpmxsia() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([990.0, 291.0], [145.0, 332.0]).len, 862.848);
}
#[test]
fn aurora_ckbnowgoba() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([596.0, 662.0], [106.0, 157.0]).len, 857.789);
}
#[test]
fn aurora_uezsnbyzjo() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([1020.0, 303.0], [279.0, 508.0]).len, 857.278);
}
#[test]
fn aurora_lfgkdffvqf() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([807.0, 100.0], [373.0, 517.0]).len, 858.409);
}
#[test]
fn aurora_sigsdauzad() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([442.0, 658.0], [420.0, 205.0]).len, 864.418);
}
#[test]
fn aurora_ijxlfimkab() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([27.0, 440.0], [844.0, 297.0]).len, 866.495);
}
#[test]
fn aurora_uerpmqcmyh() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([222.0, 586.0], [730.0, 227.0]).len, 856.371);
}
#[test]
fn aurora_zomjxasgvv() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([935.0, 564.0], [105.0, 468.0]).len, 856.26);
}
#[test]
fn aurora_ogxquacrwh() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([33.0, 497.0], [873.0, 608.0]).len, 870.469);
}
#[test]
fn aurora_ridsiqviak() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([873.0, 378.0], [40.0, 379.0]).len, 868.815);
}
#[test]
fn aurora_ylcddhhtcn() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([26.0, 594.0], [678.0, 279.0]).len, 859.465);
}
#[test]
fn aurora_ieoppledsx() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([5.0, 632.0], [645.0, 364.0]).len, 853.965);
}
#[test]
fn aurora_iegnkqhijw() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([717.0, 50.0], [941.0, 607.0]).len, 862.472);
}
#[test]
fn aurora_aspbvfcwud() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([787.0, 107.0], [339.0, 545.0]).len, 857.178);
}
#[test]
fn aurora_nfkmodptor() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([836.0, 654.0], [108.0, 209.0]).len, 857.599);
}
#[test]
fn aurora_bhjfjnxgiv() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([77.0, 170.0], [913.0, 314.0]).len, 865.426);
}
#[test]
fn aurora_evzntizmyu() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([698.0, 675.0], [406.0, 213.0]).len, 859.011);
}
#[test]
fn aurora_czzqrumgwn() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([230.0, 551.0], [973.0, 437.0]).len, 861.276);
}
#[test]
fn aurora_jmqozweiqv() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([336.0, 531.0], [764.0, 79.0]).len, 861.172);
}
#[test]
fn aurora_swdowkcvns() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([693.0, 621.0], [664.0, 87.0]).len, 863.373);
}
#[test]
fn aurora_tzwbwqiyfr() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([794.0, 716.0], [308.0, 238.0]).len, 862.795);
}
#[test]
fn aurora_qwkjjqbebl() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([566.0, 653.0], [661.0, 118.0]).len, 859.511);
}
#[test]
fn aurora_vrslqwzssi() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([348.0, 653.0], [604.0, 109.0]).len, 863.824);
}
#[test]
fn aurora_lacibxvkig() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([903.0, 137.0], [181.0, 450.0]).len, 870.11);
}
#[test]
fn aurora_kwqzyfxunu() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([826.0, 108.0], [466.0, 648.0]).len, 863.943);
}
#[test]
fn aurora_eaezvohzds() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([181.0, 689.0], [777.0, 384.0]).len, 865.801);
}
#[test]
fn aurora_vnqqtexpkw() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([974.0, 404.0], [162.0, 284.0]).len, 864.012);
}
#[test]
fn aurora_obtzwhzuiz() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([351.0, 116.0], [932.0, 595.0]).len, 864.161);
}
#[test]
fn aurora_lxfdrguftu() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([782.0, 83.0], [687.0, 674.0]).len, 873.644);
}
#[test]
fn aurora_hkbvzrrclg() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([642.0, 626.0], [59.0, 183.0]).len, 870.017);
}
#[test]
fn aurora_umsfyhqekj() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([708.0, 83.0], [625.0, 636.0]).len, 872.761);
}
#[test]
fn aurora_swukqdcjeq() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([975.0, 587.0], [124.0, 506.0]).len, 877.18);
}
#[test]
fn aurora_xfqkcddiug() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([681.0, 601.0], [82.0, 165.0]).len, 860.919);
}
#[test]
fn aurora_tzcxqkipjk() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([51.0, 398.0], [860.0, 224.0]).len, 877.405);
}
#[test]
fn aurora_poishxyunt() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([158.0, 255.0], [977.0, 464.0]).len, 867.571);
}
#[test]
fn aurora_maoyzipmpz() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([796.0, 93.0], [676.0, 684.0]).len, 871.55);
}
#[test]
fn aurora_gobymcraid() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([975.0, 580.0], [122.0, 560.0]).len, 875.655);
}
#[test]
fn aurora_dlojrxmzhf() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([619.0, 173.0], [335.0, 672.0]).len, 869.388);
}
#[test]
fn aurora_qmashfpbfr() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([50.0, 642.0], [599.0, 180.0]).len, 871.439);
}
#[test]
fn aurora_qpwcncokfl() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([663.0, 125.0], [784.0, 679.0]).len, 875.38);
}
#[test]
fn aurora_vmtayyybix() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([345.0, 241.0], [619.0, 737.0]).len, 879.752);
}
#[test]
fn aurora_wuafcpyjov() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([759.0, 172.0], [260.0, 581.0]).len, 882.386);
}
#[test]
fn aurora_vujnywdetb() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([146.0, 651.0], [816.0, 322.0]).len, 871.7);
}
#[test]
fn aurora_awkmkiocjz() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([39.0, 550.0], [709.0, 111.0]).len, 863.178);
}
#[test]
fn aurora_djfzutmygt() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([761.0, 93.0], [241.0, 560.0]).len, 869.443);
}
#[test]
fn aurora_ekijimilnl() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([708.0, 175.0], [33.0, 573.0]).len, 877.0);
}
#[test]
fn aurora_tamhztxeyu() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([746.0, 149.0], [158.0, 540.0]).len, 874.233);
}
#[test]
fn aurora_unovkotozd() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([351.0, 738.0], [350.0, 258.0]).len, 877.066);
}
#[test]
fn aurora_iwfxwtpywt() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([908.0, 584.0], [58.0, 552.0]).len, 884.749);
}
#[test]
fn aurora_qflfqtxscf() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([647.0, 723.0], [868.0, 147.0]).len, 879.643);
}
#[test]
fn aurora_ynzkscunvm() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([56.0, 667.0], [598.0, 165.0]).len, 880.736);
}
#[test]
fn aurora_yathjglwio() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([130.0, 685.0], [777.0, 400.0]).len, 873.798);
}
#[test]
fn aurora_xnfylsymeq() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([20.0, 402.0], [823.0, 645.0]).len, 872.457);
}
#[test]
fn aurora_oxlyskhdyu() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([266.0, 98.0], [796.0, 575.0]).len, 870.671);
}
#[test]
fn aurora_vkwhtibhtm() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([262.0, 582.0], [723.0, 61.0]).len, 872.43);
}
#[test]
fn aurora_yrkoibhhai() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([201.0, 576.0], [582.0, 30.0]).len, 879.565);
}
#[test]
fn aurora_puvwjtcsia() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([703.0, 260.0], [459.0, 672.0]).len, 875.169);
}
#[test]
fn aurora_pftzhybuje() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([189.0, 467.0], [997.0, 399.0]).len, 873.677);
}
#[test]
fn aurora_dtgisybkdy() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([15.0, 352.0], [882.0, 253.0]).len, 891.072);
}
#[test]
fn aurora_utgdioksjo() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([430.0, 224.0], [858.0, 629.0]).len, 881.87);
}
#[test]
fn aurora_wkiamzlypj() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([459.0, 665.0], [314.0, 92.0]).len, 891.913);
}
#[test]
fn aurora_lqakjgkvpd() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([317.0, 620.0], [687.0, 163.0]).len, 886.015);
}
#[test]
fn aurora_jqijvmmrvb() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([358.0, 699.0], [305.0, 93.0]).len, 883.091);
}
#[test]
fn aurora_rpcpupdvvr() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([121.0, 375.0], [991.0, 291.0]).len, 887.857);
}
#[test]
fn aurora_kzslumltzq() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([766.0, 713.0], [787.0, 77.0]).len, 887.072);
}
#[test]
fn aurora_rmthpykhiv() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([605.0, 173.0], [522.0, 708.0]).len, 885.674);
}
#[test]
fn aurora_fcxrzgwsmg() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([23.0, 632.0], [586.0, 161.0]).len, 875.728);
}
#[test]
fn aurora_svbqhejwvo() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([718.0, 91.0], [189.0, 594.0]).len, 877.376);
}
#[test]
fn aurora_fcluoorlqa() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([724.0, 637.0], [174.0, 146.0]).len, 879.025);
}
#[test]
fn aurora_oidmahtqie() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([334.0, 57.0], [251.0, 711.0]).len, 888.721);
}
#[test]
fn aurora_fajgqpqufm() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([136.0, 131.0], [943.0, 391.0]).len, 879.822);
}
#[test]
fn aurora_dnsgmczczu() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([505.0, 663.0], [754.0, 267.0]).len, 885.578);
}
#[test]
fn aurora_pesixvdcdl() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([31.0, 450.0], [821.0, 401.0]).len, 879.519);
}
#[test]
fn aurora_sgdcqknguq() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([13.0, 190.0], [807.0, 544.0]).len, 876.332);
}
#[test]
fn aurora_xvjxqclumu() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([231.0, 679.0], [455.0, 58.0]).len, 893.715);
}
#[test]
fn aurora_milprbplao() {
    let aurora = aurora_mesh();
    known_failure!(aurora.path([754.0, 697.0], [609.0, 160.0]).len, 884.499);
}
#[test]
fn aurora_glzilivxrn() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([782.0, 68.0], [481.0, 615.0]).len, 885.976);
}
#[test]
fn aurora_amqhfkbqxn() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([927.0, 334.0], [96.0, 468.0]).len, 890.27);
}
#[test]
fn aurora_kmigostwdv() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([772.0, 215.0], [382.0, 626.0]).len, 885.467);
}
#[test]
fn aurora_loboiwaafs() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([181.0, 527.0], [1023.0, 605.0]).len, 886.561);
}
#[test]
fn aurora_wxwwhoxaxy() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([993.0, 299.0], [290.0, 657.0]).len, 896.545);
}
#[test]
fn aurora_vurchgqpit() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([731.0, 381.0], [4.0, 612.0]).len, 881.403);
}
#[test]
fn aurora_akvpgtuvbq() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([43.0, 178.0], [650.0, 624.0]).len, 889.061);
}
#[test]
fn aurora_nwvzdumdfx() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([807.0, 84.0], [278.0, 533.0]).len, 882.031);
}
#[test]
fn aurora_zxctkxfhoe() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([650.0, 192.0], [365.0, 615.0]).len, 881.064);
}
#[test]
fn aurora_arvhiidmec() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([224.0, 61.0], [619.0, 577.0]).len, 882.891);
}
#[test]
fn aurora_olfscfouqy() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([290.0, 640.0], [986.0, 298.0]).len, 896.034);
}
#[test]
fn aurora_ipqzhnqcwx() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([607.0, 117.0], [1016.0, 613.0]).len, 879.512);
}
#[test]
fn aurora_qbticsdeca() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([98.0, 703.0], [594.0, 180.0]).len, 890.441);
}
#[test]
fn aurora_qgbghjhtyg() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([177.0, 148.0], [383.0, 715.0]).len, 890.264);
}
#[test]
fn aurora_bsnxxukpqc() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([284.0, 730.0], [594.0, 192.0]).len, 894.09);
}
#[test]
fn aurora_spkurzamqw() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([857.0, 324.0], [30.0, 514.0]).len, 896.472);
}
#[test]
fn aurora_gkqugoxhue() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([39.0, 188.0], [912.0, 305.0]).len, 902.743);
}
#[test]
fn aurora_vbyiwxollk() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([397.0, 605.0], [878.0, 152.0]).len, 888.26);
}
#[test]
fn aurora_kpjirjsvvy() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([140.0, 250.0], [1014.0, 297.0]).len, 898.081);
}
#[test]
fn aurora_juabubrrqw() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([607.0, 713.0], [105.0, 213.0]).len, 893.597);
}
#[test]
fn aurora_ryofopwtxi() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([185.0, 634.0], [701.0, 115.0]).len, 889.437);
}
#[test]
fn aurora_hvgubouwwk() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([900.0, 470.0], [36.0, 494.0]).len, 896.375);
}
#[test]
fn aurora_snhkbndmmq() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([984.0, 625.0], [701.0, 67.0]).len, 893.75);
}
#[test]
fn aurora_uuecguzweo() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([244.0, 716.0], [924.0, 330.0]).len, 901.896);
}
#[test]
fn aurora_aaoxhytbrk() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([879.0, 542.0], [19.0, 628.0]).len, 897.147);
}
#[test]
fn aurora_hkrdomhjwv() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([388.0, 264.0], [407.0, 717.0]).len, 897.799);
}
#[test]
fn aurora_avwxfmnryq() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([152.0, 570.0], [894.0, 308.0]).len, 886.808);
}
#[test]
fn aurora_kcsfbewpvq() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([637.0, 690.0], [348.0, 175.0]).len, 894.243);
}
#[test]
fn aurora_ygxemnrmbw() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([946.0, 647.0], [307.0, 268.0]).len, 888.517);
}
#[test]
fn aurora_kevvvgczjf() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([181.0, 695.0], [353.0, 121.0]).len, 895.829);
}
#[test]
fn aurora_ajdpycbfzt() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([106.0, 597.0], [978.0, 623.0]).len, 903.431);
}
#[test]
fn aurora_kmenbzvjqt() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([835.0, 379.0], [148.0, 663.0]).len, 892.344);
}
#[test]
fn aurora_cysehndvxx() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([995.0, 633.0], [180.0, 298.0]).len, 890.108);
}
#[test]
fn aurora_wxysmcsqli() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([763.0, 673.0], [16.0, 439.0]).len, 893.83);
}
#[test]
fn aurora_iltprxjvhf() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([843.0, 368.0], [152.0, 667.0]).len, 900.312);
}
#[test]
fn aurora_udwntlvjzt() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([19.0, 633.0], [739.0, 304.0]).len, 895.488);
}
#[test]
fn aurora_vselobxrlh() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([867.0, 367.0], [192.0, 655.0]).len, 901.562);
}
#[test]
fn aurora_agsdhuneir() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([112.0, 477.0], [984.0, 612.0]).len, 904.843);
}
#[test]
fn aurora_lqmqyqathd() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([78.0, 336.0], [892.0, 620.0]).len, 891.612);
}
#[test]
fn aurora_nbguocoehl() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([952.0, 599.0], [82.0, 531.0]).len, 901.449);
}
#[test]
fn aurora_babuykyapb() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([235.0, 635.0], [624.0, 50.0]).len, 902.652);
}
#[test]
fn aurora_iuwdfcrgbh() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([889.0, 163.0], [358.0, 666.0]).len, 907.67);
}
#[test]
fn aurora_ahefemacer() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([408.0, 685.0], [291.0, 86.0]).len, 904.762);
}
#[test]
fn aurora_jddlkansue() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([649.0, 706.0], [739.0, 107.0]).len, 903.56);
}
#[test]
fn aurora_tvcdndsnzs() {
    let aurora = aurora_mesh();
    known_failure!(aurora.path([871.0, 686.0], [376.0, 91.0]).len, 899.012);
}
#[test]
fn aurora_arfbnfqluz() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([42.0, 656.0], [769.0, 380.0]).len, 896.515);
}
#[test]
fn aurora_xkuqrinfos() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([563.0, 695.0], [447.0, 140.0]).len, 898.006);
}
#[test]
fn aurora_gvqkuohkoo() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([219.0, 201.0], [816.0, 601.0]).len, 897.739);
}
#[test]
fn aurora_ezynkeifmt() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([531.0, 713.0], [731.0, 224.0]).len, 901.315);
}
#[test]
fn aurora_xqvfyqzcsx() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([690.0, 140.0], [512.0, 684.0]).len, 898.337);
}
#[test]
fn aurora_ypzfeinwxr() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([673.0, 694.0], [758.0, 76.0]).len, 906.397);
}
#[test]
fn aurora_ibmacvxkny() {
    let aurora = aurora_mesh();
    known_failure!(aurora.path([930.0, 588.0], [336.0, 56.0]).len, 899.331);
}
#[test]
fn aurora_kikvouhegs() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([185.0, 705.0], [909.0, 374.0]).len, 916.128);
}
#[test]
fn aurora_ecbwjynlgg() {
    let aurora = aurora_mesh();
    known_failure!(aurora.path([815.0, 472.0], [108.0, 195.0]).len, 906.041);
}
#[test]
fn aurora_wwoxesqgta() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([722.0, 57.0], [197.0, 609.0]).len, 904.248);
}
#[test]
fn aurora_lgwzgcwhdy() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([33.0, 509.0], [787.0, 199.0]).len, 907.722);
}
#[test]
fn aurora_lznoxaytak() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([36.0, 681.0], [928.0, 608.0]).len, 925.866);
}
#[test]
fn aurora_mkevvuezhh() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([958.0, 603.0], [308.0, 308.0]).len, 898.636);
}
#[test]
fn aurora_avanelxoly() {
    let aurora = aurora_mesh();
    known_failure!(aurora.path([859.0, 484.0], [8.0, 359.0]).len, 906.059);
}
#[test]
fn aurora_jewbzssibd() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([713.0, 259.0], [75.0, 496.0]).len, 897.479);
}
#[test]
fn aurora_nqqkgbmkca() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([576.0, 706.0], [743.0, 236.0]).len, 907.127);
}
#[test]
fn aurora_ycjmhuhzgn() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([188.0, 181.0], [341.0, 732.0]).len, 902.097);
}
#[test]
fn aurora_bjxngmqcey() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([36.0, 442.0], [889.0, 252.0]).len, 918.387);
}
#[test]
fn aurora_pmbsvwfbcs() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([924.0, 162.0], [387.0, 641.0]).len, 910.014);
}
#[test]
fn aurora_bfwpwtsbym() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([721.0, 223.0], [273.0, 631.0]).len, 906.289);
}
#[test]
fn aurora_sccyfrrbbf() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([276.0, 270.0], [1008.0, 602.0]).len, 901.5);
}
#[test]
fn aurora_bobzulagyy() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([11.0, 181.0], [821.0, 563.0]).len, 899.808);
}
#[test]
fn aurora_ytlhjivrzp() {
    let aurora = aurora_mesh();
    known_failure!(aurora.path([792.0, 598.0], [282.0, 43.0]).len, 905.102);
}
#[test]
fn aurora_fcudmsukay() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([786.0, 259.0], [16.0, 444.0]).len, 910.057);
}
#[test]
fn aurora_bgoiiefqko() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([355.0, 610.0], [676.0, 117.0]).len, 903.274);
}
#[test]
fn aurora_holbdaroaw() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([499.0, 683.0], [772.0, 246.0]).len, 904.315);
}
#[test]
fn aurora_yqhgwjkvis() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([140.0, 541.0], [923.0, 384.0]).len, 911.702);
}
#[test]
fn aurora_jalhvquzhv() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([824.0, 124.0], [270.0, 540.0]).len, 907.684);
}
#[test]
fn aurora_ssxrriyevv() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([110.0, 273.0], [1002.0, 365.0]).len, 923.216);
}
#[test]
fn aurora_obimnzsrzi() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([441.0, 695.0], [423.0, 91.0]).len, 922.328);
}
#[test]
fn aurora_urbcgdwoqe() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([730.0, 688.0], [134.0, 196.0]).len, 910.519);
}
#[test]
fn aurora_xdrxwisipy() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([298.0, 121.0], [883.0, 622.0]).len, 909.531);
}
#[test]
fn aurora_wtutdbrgbh() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([137.0, 113.0], [447.0, 670.0]).len, 910.983);
}
#[test]
fn aurora_csanlkblbw() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([744.0, 644.0], [332.0, 94.0]).len, 909.436);
}
#[test]
fn aurora_xyehqdgqfe() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([663.0, 676.0], [245.0, 160.0]).len, 916.703);
}
#[test]
fn aurora_dolfiqkgzm() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([734.0, 146.0], [320.0, 597.0]).len, 915.57);
}
#[test]
fn aurora_htwgrbgyvl() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([120.0, 632.0], [844.0, 322.0]).len, 906.742);
}
#[test]
fn aurora_pesmnngifx() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([715.0, 552.0], [198.0, 73.0]).len, 908.876);
}
#[test]
fn aurora_goubljtquu() {
    let aurora = aurora_mesh();
    known_failure!(aurora.path([829.0, 664.0], [508.0, 34.0]).len, 921.456);
}
#[test]
fn aurora_enelnvfjaa() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([68.0, 569.0], [835.0, 313.0]).len, 909.053);
}
#[test]
fn aurora_emxhfjvmsp() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([323.0, 175.0], [765.0, 715.0]).len, 918.319);
}
#[test]
fn aurora_qjalhpqzhf() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([186.0, 70.0], [998.0, 414.0]).len, 907.793);
}
#[test]
fn aurora_gxqiitpega() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([160.0, 576.0], [910.0, 278.0]).len, 908.755);
}
#[test]
fn aurora_naicnxxsjr() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([889.0, 364.0], [12.0, 182.0]).len, 932.431);
}
#[test]
fn aurora_zdgvhqvygm() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([146.0, 696.0], [647.0, 88.0]).len, 918.124);
}
#[test]
fn aurora_coqhcuvpmq() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([380.0, 637.0], [667.0, 163.0]).len, 919.247);
}
#[test]
fn aurora_ilunuxawqv() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([69.0, 488.0], [935.0, 421.0]).len, 918.877);
}
#[test]
fn aurora_jahbfdlsnk() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([918.0, 551.0], [14.0, 598.0]).len, 928.754);
}
#[test]
fn aurora_qkhogepcnv() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([252.0, 724.0], [939.0, 312.0]).len, 926.417);
}
#[test]
fn aurora_rsqibgsyqa() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([426.0, 690.0], [700.0, 180.0]).len, 917.882);
}
#[test]
fn aurora_npjyucikld() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([28.0, 507.0], [940.0, 564.0]).len, 930.844);
}
#[test]
fn aurora_adpshgcpke() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([883.0, 576.0], [16.0, 391.0]).len, 917.343);
}
#[test]
fn aurora_oymqpeswaq() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([380.0, 129.0], [791.0, 733.0]).len, 927.124);
}
#[test]
fn aurora_ldlfnugkss() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([693.0, 118.0], [178.0, 671.0]).len, 918.549);
}
#[test]
fn aurora_ireqilqfxc() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([706.0, 81.0], [150.0, 629.0]).len, 919.708);
}
#[test]
fn aurora_srzghbprjj() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([738.0, 50.0], [746.0, 660.0]).len, 925.589);
}
#[test]
fn aurora_mtkpnkznqx() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([100.0, 597.0], [724.0, 168.0]).len, 928.468);
}
#[test]
fn aurora_tfhtucmwqc() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([529.0, 703.0], [870.0, 121.0]).len, 928.03);
}
#[test]
fn aurora_jflfgjrrjm() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([593.0, 717.0], [452.0, 169.0]).len, 926.867);
}
#[test]
fn aurora_vyueslwrlf() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([949.0, 646.0], [71.0, 453.0]).len, 925.038);
}
#[test]
fn aurora_uvqpbcmcuz() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([623.0, 221.0], [128.0, 726.0]).len, 924.834);
}
#[test]
fn aurora_ztnenoklnh() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([31.0, 645.0], [637.0, 178.0]).len, 925.962);
}
#[test]
fn aurora_vwfvitgeii() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([864.0, 186.0], [190.0, 551.0]).len, 921.604);
}
#[test]
fn aurora_lcicnldrps() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([850.0, 359.0], [117.0, 668.0]).len, 931.373);
}
#[test]
fn aurora_wiudmugjgq() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([1008.0, 387.0], [133.0, 157.0]).len, 929.572);
}
#[test]
fn aurora_mhdcnzbkuv() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([219.0, 220.0], [650.0, 715.0]).len, 933.76);
}
#[test]
fn aurora_hxzgfuqovm() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([121.0, 422.0], [981.0, 400.0]).len, 922.686);
}
#[test]
fn aurora_ucvfzgfrrb() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([19.0, 357.0], [873.0, 180.0]).len, 927.62);
}
#[test]
fn aurora_ztpyllrevu() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([923.0, 274.0], [11.0, 161.0]).len, 941.495);
}
#[test]
fn aurora_lqorzhlnvp() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([183.0, 708.0], [654.0, 172.0]).len, 937.367);
}
#[test]
fn aurora_trpyckypzg() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([887.0, 653.0], [31.0, 392.0]).len, 924.575);
}
#[test]
fn aurora_fkflkojkgw() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([277.0, 126.0], [903.0, 618.0]).len, 924.113);
}
#[test]
fn aurora_vgoityctru() {
    let aurora = aurora_mesh();
    known_failure!(aurora.path([867.0, 479.0], [125.0, 185.0]).len, 931.005);
}
#[test]
fn aurora_dvzuizkfjf() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([19.0, 370.0], [879.0, 614.0]).len, 930.969);
}
#[test]
fn aurora_cplegltvgw() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([77.0, 162.0], [747.0, 635.0]).len, 922.251);
}
#[test]
fn aurora_sxiechunjo() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([952.0, 276.0], [214.0, 585.0]).len, 921.092);
}
#[test]
fn aurora_bbmqfemhhi() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([959.0, 642.0], [73.0, 463.0]).len, 931.176);
}
#[test]
fn aurora_wvotqeupre() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([388.0, 212.0], [615.0, 731.0]).len, 937.626);
}
#[test]
fn aurora_tokirieklf() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([943.0, 608.0], [23.0, 521.0]).len, 943.609);
}
#[test]
fn aurora_ofhctdcwav() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([86.0, 557.0], [736.0, 161.0]).len, 937.368);
}
#[test]
fn aurora_yoyeifhecr() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([335.0, 678.0], [899.0, 280.0]).len, 931.922);
}
#[test]
fn aurora_mqdrqodlxx() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([975.0, 633.0], [68.0, 684.0]).len, 936.786);
}
#[test]
fn aurora_xvfjbnfrij() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([952.0, 455.0], [58.0, 601.0]).len, 932.185);
}
#[test]
fn aurora_lvfgccehry() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([512.0, 41.0], [56.0, 638.0]).len, 935.507);
}
#[test]
fn aurora_clmgiimatj() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([83.0, 460.0], [889.0, 136.0]).len, 937.151);
}
#[test]
fn aurora_sxozuvtdsf() {
    let aurora = aurora_mesh();
    known_failure!(aurora.path([896.0, 534.0], [189.0, 248.0]).len, 930.747);
}
#[test]
fn aurora_gliinkprvy() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([761.0, 149.0], [556.0, 707.0]).len, 930.898);
}
#[test]
fn aurora_zmtphsnmda() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([816.0, 418.0], [44.0, 595.0]).len, 928.164);
}
#[test]
fn aurora_beebjbfgmk() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([969.0, 273.0], [68.0, 454.0]).len, 942.777);
}
#[test]
fn aurora_dquusnusfi() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([125.0, 703.0], [663.0, 139.0]).len, 935.318);
}
#[test]
fn aurora_htnlfclnkk() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([45.0, 484.0], [904.0, 420.0]).len, 941.597);
}
#[test]
fn aurora_tstewarual() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([903.0, 305.0], [112.0, 510.0]).len, 930.391);
}
#[test]
fn aurora_zcvbhknebb() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([744.0, 238.0], [118.0, 503.0]).len, 925.752);
}
#[test]
fn aurora_yowiqqrrwn() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([9.0, 632.0], [787.0, 369.0]).len, 928.191);
}
#[test]
fn aurora_efxtvryumr() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([320.0, 265.0], [992.0, 598.0]).len, 930.328);
}
#[test]
fn aurora_czliddeadv() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([43.0, 370.0], [977.0, 283.0]).len, 949.478);
}
#[test]
fn aurora_nnvodajcut() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([703.0, 174.0], [415.0, 699.0]).len, 932.79);
}
#[test]
fn aurora_ermxuvucah() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([437.0, 67.0], [660.0, 713.0]).len, 946.789);
}
#[test]
fn aurora_bkozntwvax() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([841.0, 233.0], [28.0, 508.0]).len, 946.75);
}
#[test]
fn aurora_gggdgckrwq() {
    let aurora = aurora_mesh();
    known_failure!(aurora.path([736.0, 628.0], [597.0, 31.0]).len, 940.314);
}
#[test]
fn aurora_rpapckdeab() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([195.0, 666.0], [943.0, 338.0]).len, 954.407);
}
#[test]
fn aurora_unbkxwsdhi() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([873.0, 580.0], [198.0, 233.0]).len, 928.666);
}
#[test]
fn aurora_tvixslodwi() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([205.0, 495.0], [923.0, 149.0]).len, 943.375);
}
#[test]
fn aurora_dqvwvaaxlm() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([913.0, 551.0], [22.0, 366.0]).len, 944.993);
}
#[test]
fn aurora_yuhqwztzwu() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([741.0, 716.0], [277.0, 141.0]).len, 940.238);
}
#[test]
fn aurora_sxroinxuky() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([438.0, 658.0], [336.0, 72.0]).len, 939.028);
}
#[test]
fn aurora_vydlmotcbs() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([130.0, 646.0], [696.0, 70.0]).len, 939.846);
}
#[test]
fn aurora_mekfuimdkm() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([35.0, 459.0], [879.0, 413.0]).len, 948.28);
}
#[test]
fn aurora_gxdbxvtwtn() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([135.0, 642.0], [734.0, 140.0]).len, 951.166);
}
#[test]
fn aurora_ysakcrgnzw() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([304.0, 658.0], [817.0, 232.0]).len, 942.483);
}
#[test]
fn aurora_qutcyiovjf() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([262.0, 33.0], [748.0, 599.0]).len, 940.324);
}
#[test]
fn aurora_xckesugacp() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([848.0, 125.0], [4.0, 449.0]).len, 959.249);
}
#[test]
fn aurora_ifxabtwsir() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([859.0, 616.0], [218.0, 240.0]).len, 937.642);
}
#[test]
fn aurora_msuqbrjbjz() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([836.0, 93.0], [565.0, 709.0]).len, 946.308);
}
#[test]
fn aurora_jwsydnecbi() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([797.0, 543.0], [91.0, 153.0]).len, 935.854);
}
#[test]
fn aurora_zkzsxmggxl() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([926.0, 610.0], [66.0, 354.0]).len, 936.27);
}
#[test]
fn aurora_hqqsctxasp() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([880.0, 363.0], [30.0, 535.0]).len, 950.006);
}
#[test]
fn aurora_snqlewuqgq() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([902.0, 419.0], [46.0, 581.0]).len, 950.845);
}
#[test]
fn aurora_mjcoiemnxg() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([137.0, 311.0], [1011.0, 609.0]).len, 940.191);
}
#[test]
fn aurora_bgoqzgimjy() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([160.0, 621.0], [754.0, 99.0]).len, 947.641);
}
#[test]
fn aurora_nppzvxmxpa() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([992.0, 297.0], [69.0, 175.0]).len, 954.016);
}
#[test]
fn aurora_ydnoallghu() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([536.0, 27.0], [326.0, 624.0]).len, 949.066);
}
#[test]
fn aurora_arqkrdqkau() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([599.0, 207.0], [405.0, 706.0]).len, 949.642);
}
#[test]
fn aurora_pcboaoerra() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([638.0, 707.0], [406.0, 84.0]).len, 951.753);
}
#[test]
fn aurora_tmuoyhmykq() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([971.0, 414.0], [91.0, 211.0]).len, 956.628);
}
#[test]
fn aurora_eghmfnxgyv() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([835.0, 709.0], [33.0, 463.0]).len, 941.099);
}
#[test]
fn aurora_bhdbruzndy() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([563.0, 178.0], [361.0, 722.0]).len, 950.743);
}
#[test]
fn aurora_ptdzgrhwha() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([258.0, 148.0], [627.0, 701.0]).len, 955.041);
}
#[test]
fn aurora_irwifgshaw() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([428.0, 246.0], [765.0, 682.0]).len, 946.053);
}
#[test]
fn aurora_xmkwtquqos() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([118.0, 447.0], [1017.0, 373.0]).len, 958.489);
}
#[test]
fn aurora_gokssskaox() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([13.0, 504.0], [807.0, 180.0]).len, 946.135);
}
#[test]
fn aurora_bqlzmbgaws() {
    let aurora = aurora_mesh();
    known_failure!(aurora.path([958.0, 616.0], [305.0, 92.0]).len, 948.846);
}
#[test]
fn aurora_mwjbsodaau() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([990.0, 366.0], [116.0, 508.0]).len, 955.236);
}
#[test]
fn aurora_trtrzxngpm() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([130.0, 509.0], [1000.0, 351.0]).len, 959.488);
}
#[test]
fn aurora_sbusyyoycn() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([820.0, 213.0], [317.0, 660.0]).len, 951.569);
}
#[test]
fn aurora_whtxssrqit() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([876.0, 449.0], [16.0, 465.0]).len, 943.985);
}
#[test]
fn aurora_dyujgrohsw() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([573.0, 697.0], [386.0, 88.0]).len, 950.723);
}
#[test]
fn aurora_btursqsjbu() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([95.0, 134.0], [541.0, 709.0]).len, 957.867);
}
#[test]
fn aurora_mehgjlozoe() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([54.0, 374.0], [947.0, 580.0]).len, 953.564);
}
#[test]
fn aurora_gszmyzzywa() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([826.0, 616.0], [152.0, 153.0]).len, 954.121);
}
#[test]
fn aurora_xxaslddvla() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([72.0, 577.0], [719.0, 225.0]).len, 949.135);
}
#[test]
fn aurora_wagszfgdls() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([62.0, 528.0], [908.0, 339.0]).len, 960.329);
}
#[test]
fn aurora_dcwwlicutz() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([222.0, 235.0], [947.0, 557.0]).len, 951.591);
}
#[test]
fn aurora_jdkwwarscr() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([808.0, 140.0], [254.0, 611.0]).len, 961.662);
}
#[test]
fn aurora_uypulgkyon() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([756.0, 698.0], [28.0, 198.0]).len, 953.558);
}
#[test]
fn aurora_dpnjfbqcke() {
    let aurora = aurora_mesh();
    known_failure!(aurora.path([931.0, 568.0], [199.0, 208.0]).len, 963.295);
}
#[test]
fn aurora_dhzaqvhzto() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([275.0, 568.0], [814.0, 109.0]).len, 951.806);
}
#[test]
fn aurora_rrtxftvsme() {
    let aurora = aurora_mesh();
    known_failure!(aurora.path([756.0, 671.0], [349.0, 66.0]).len, 961.606);
}
#[test]
fn aurora_mtcoakaykv() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([434.0, 123.0], [379.0, 722.0]).len, 962.82);
}
#[test]
fn aurora_hvpfripjtu() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([397.0, 254.0], [731.0, 682.0]).len, 955.612);
}
#[test]
fn aurora_zoqmaorihd() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([882.0, 278.0], [206.0, 704.0]).len, 960.196);
}
#[test]
fn aurora_xoiiowqgsb() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([196.0, 589.0], [835.0, 155.0]).len, 969.113);
}
#[test]
fn aurora_hkcvvaxefl() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([225.0, 170.0], [760.0, 691.0]).len, 957.338);
}
#[test]
fn aurora_hexylsfqrk() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([27.0, 200.0], [750.0, 684.0]).len, 952.049);
}
#[test]
fn aurora_twrykljxbi() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([999.0, 358.0], [118.0, 509.0]).len, 965.04);
}
#[test]
fn aurora_qwaiddygtq() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([914.0, 532.0], [14.0, 635.0]).len, 959.281);
}
#[test]
fn aurora_qhxnfusoxl() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([831.0, 235.0], [7.0, 473.0]).len, 962.378);
}
#[test]
fn aurora_vkcxkfjkoo() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([945.0, 407.0], [42.0, 159.0]).len, 967.566);
}
#[test]
fn aurora_lvnouffvdd() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([49.0, 517.0], [938.0, 263.0]).len, 967.541);
}
#[test]
fn aurora_ptfcbpsfdp() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([350.0, 621.0], [747.0, 148.0]).len, 966.31);
}
#[test]
fn aurora_qrmufvjxgw() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([734.0, 258.0], [103.0, 594.0]).len, 958.748);
}
#[test]
fn aurora_svdfipwqng() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([172.0, 640.0], [909.0, 254.0]).len, 955.79);
}
#[test]
fn aurora_jfmdzcxacl() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([600.0, 709.0], [242.0, 180.0]).len, 964.472);
}
#[test]
fn aurora_vehtbfsios() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([65.0, 185.0], [978.0, 377.0]).len, 967.203);
}
#[test]
fn aurora_icyfhdbtou() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([125.0, 474.0], [1015.0, 421.0]).len, 961.996);
}
#[test]
fn aurora_vuomglxwkx() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([24.0, 172.0], [942.0, 362.0]).len, 970.023);
}
#[test]
fn aurora_hzhonvcaxj() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([926.0, 380.0], [139.0, 698.0]).len, 971.334);
}
#[test]
fn aurora_vwgzwoszqk() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([993.0, 632.0], [61.0, 467.0]).len, 971.354);
}
#[test]
fn aurora_exnidyojvk() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([173.0, 226.0], [888.0, 673.0]).len, 960.863);
}
#[test]
fn aurora_opfcpwklfe() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([8.0, 365.0], [919.0, 582.0]).len, 967.566);
}
#[test]
fn aurora_zlnkhvnxds() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([790.0, 90.0], [138.0, 560.0]).len, 961.117);
}
#[test]
fn aurora_imcemzoief() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([352.0, 715.0], [812.0, 169.0]).len, 969.844);
}
#[test]
fn aurora_zoewyasqpm() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([207.0, 709.0], [679.0, 69.0]).len, 972.769);
}
#[test]
fn aurora_xxqpgdykff() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([891.0, 172.0], [217.0, 587.0]).len, 963.013);
}
#[test]
fn aurora_tknwvmdckl() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([969.0, 631.0], [28.0, 493.0]).len, 974.411);
}
#[test]
fn aurora_cagrgnpcuo() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([500.0, 47.0], [230.0, 719.0]).len, 970.065);
}
#[test]
fn aurora_gqsoifjwoc() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([854.0, 200.0], [322.0, 679.0]).len, 968.262);
}
#[test]
fn aurora_knzjmvwidy() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([324.0, 742.0], [61.0, 147.0]).len, 968.343);
}
#[test]
fn aurora_nrbthxylke() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([738.0, 260.0], [88.0, 567.0]).len, 964.753);
}
#[test]
fn aurora_jnfmzhfpkt() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([203.0, 694.0], [717.0, 153.0]).len, 978.264);
}
#[test]
fn aurora_mhxjyurjme() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([663.0, 87.0], [153.0, 735.0]).len, 969.579);
}
#[test]
fn aurora_xjxbotdlvr() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([548.0, 703.0], [365.0, 66.0]).len, 977.028);
}
#[test]
fn aurora_bvetwyjklf() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([877.0, 178.0], [52.0, 481.0]).len, 973.749);
}
#[test]
fn aurora_icqhtzhdlm() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([930.0, 622.0], [31.0, 365.0]).len, 968.832);
}
#[test]
fn aurora_jixvogfahb() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([26.0, 632.0], [947.0, 459.0]).len, 966.846);
}
#[test]
fn aurora_etagffvnks() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([888.0, 410.0], [8.0, 512.0]).len, 969.213);
}
#[test]
fn aurora_ioifztcxzk() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([308.0, 52.0], [961.0, 618.0]).len, 967.038);
}
#[test]
fn aurora_xlajdowydd() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([48.0, 546.0], [846.0, 153.0]).len, 984.055);
}
#[test]
fn aurora_zpgxwgyprf() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([833.0, 641.0], [139.0, 184.0]).len, 964.505);
}
#[test]
fn aurora_wroqupreph() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([349.0, 718.0], [935.0, 148.0]).len, 976.227);
}
#[test]
fn aurora_titfkyqunk() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([1.0, 442.0], [927.0, 381.0]).len, 986.434);
}
#[test]
fn aurora_agjjwtybua() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([710.0, 157.0], [80.0, 660.0]).len, 980.388);
}
#[test]
fn aurora_atkorxmgpt() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([950.0, 440.0], [139.0, 633.0]).len, 988.294);
}
#[test]
fn aurora_ssmdzbdsut() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([803.0, 156.0], [203.0, 635.0]).len, 984.952);
}
#[test]
fn aurora_fswhewkcuu() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([797.0, 265.0], [7.0, 520.0]).len, 969.203);
}
#[test]
fn aurora_rlfqllsdlp() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([513.0, 143.0], [312.0, 755.0]).len, 977.564);
}
#[test]
fn aurora_lyewvynstr() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([1006.0, 588.0], [51.0, 541.0]).len, 982.381);
}
#[test]
fn aurora_nybmduhizj() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([1012.0, 444.0], [58.0, 516.0]).len, 982.302);
}
#[test]
fn aurora_dyvisjkblk() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([124.0, 713.0], [983.0, 361.0]).len, 992.807);
}
#[test]
fn aurora_bpwmficxuz() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([725.0, 127.0], [243.0, 707.0]).len, 982.288);
}
#[test]
fn aurora_jisyafxfff() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([782.0, 195.0], [275.0, 646.0]).len, 979.082);
}
#[test]
fn aurora_ppguccegeu() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([596.0, 728.0], [791.0, 78.0]).len, 977.525);
}
#[test]
fn aurora_xtismmsdii() {
    let aurora = aurora_mesh();
    known_failure!(aurora.path([979.0, 589.0], [269.0, 100.0]).len, 968.371);
}
#[test]
fn aurora_nulmetxbxy() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([625.0, 128.0], [403.0, 713.0]).len, 979.638);
}
#[test]
fn aurora_chipjfxwns() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([795.0, 263.0], [261.0, 642.0]).len, 981.232);
}
#[test]
fn aurora_bktulyimxr() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([106.0, 612.0], [738.0, 282.0]).len, 977.24);
}
#[test]
fn aurora_qfhrsnxhap() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([106.0, 624.0], [760.0, 119.0]).len, 983.087);
}
#[test]
fn aurora_fwckfthhkg() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([803.0, 152.0], [123.0, 599.0]).len, 990.046);
}
#[test]
fn aurora_pbynqgcskc() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([42.0, 566.0], [793.0, 275.0]).len, 980.163);
}
#[test]
fn aurora_wldpudlvpm() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([979.0, 363.0], [83.0, 496.0]).len, 985.647);
}
#[test]
fn aurora_zohwlmtnhp() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([857.0, 598.0], [185.0, 131.0]).len, 978.562);
}
#[test]
fn aurora_vccsbrynja() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([823.0, 127.0], [8.0, 502.0]).len, 971.566);
}
#[test]
fn aurora_rlqjrofdes() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([833.0, 105.0], [397.0, 683.0]).len, 985.181);
}
#[test]
fn aurora_noryvyaefl() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([398.0, 125.0], [369.0, 713.0]).len, 989.761);
}
#[test]
fn aurora_ndvjmcwral() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([140.0, 585.0], [799.0, 87.0]).len, 976.889);
}
#[test]
fn aurora_watzphdunz() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([834.0, 329.0], [24.0, 646.0]).len, 971.856);
}
#[test]
fn aurora_kiiqxrqgbc() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([702.0, 214.0], [171.0, 700.0]).len, 978.893);
}
#[test]
fn aurora_cahnnicmup() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([849.0, 430.0], [28.0, 598.0]).len, 978.599);
}
#[test]
fn aurora_eccjdvirhb() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([792.0, 97.0], [131.0, 603.0]).len, 986.148);
}
#[test]
fn aurora_nwmgyfdrcb() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([650.0, 57.0], [722.0, 685.0]).len, 990.38);
}
#[test]
fn aurora_tvfbkyetoh() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([964.0, 349.0], [135.0, 675.0]).len, 995.156);
}
#[test]
fn aurora_lhueoiyljj() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([283.0, 685.0], [580.0, 44.0]).len, 984.101);
}
#[test]
fn aurora_ebpocctvdw() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([20.0, 171.0], [769.0, 718.0]).len, 981.948);
}
#[test]
fn aurora_inmqupmkfr() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([1016.0, 381.0], [142.0, 680.0]).len, 994.584);
}
#[test]
fn aurora_vymryuiknw() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([939.0, 357.0], [99.0, 602.0]).len, 986.219);
}
#[test]
fn aurora_irlpfkpbur() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([748.0, 145.0], [287.0, 693.0]).len, 991.334);
}
#[test]
fn aurora_xvhkzdatqd() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([339.0, 612.0], [757.0, 82.0]).len, 978.567);
}
#[test]
fn aurora_ozvjcnhjzu() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([282.0, 692.0], [743.0, 126.0]).len, 990.931);
}
#[test]
fn aurora_pbllfkzmia() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([391.0, 721.0], [374.0, 107.0]).len, 997.016);
}
#[test]
fn aurora_aseeuawdvw() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([301.0, 97.0], [745.0, 671.0]).len, 991.751);
}
#[test]
fn aurora_lvvqesqxlt() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([678.0, 249.0], [178.0, 722.0]).len, 987.037);
}
#[test]
fn aurora_dcvkpoguwo() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([198.0, 681.0], [728.0, 210.0]).len, 991.765);
}
#[test]
fn aurora_szibdgnhae() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([699.0, 163.0], [289.0, 725.0]).len, 995.027);
}
#[test]
fn aurora_zmisxsbyht() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([899.0, 339.0], [54.0, 578.0]).len, 984.014);
}
#[test]
fn aurora_ppcifuddbn() {
    let aurora = aurora_mesh();
    known_failure!(aurora.path([959.0, 641.0], [245.0, 234.0]).len, 995.78);
}
#[test]
fn aurora_wuzgxgoiac() {
    let aurora = aurora_mesh();
    known_failure!(aurora.path([924.0, 640.0], [238.0, 134.0]).len, 986.483);
}
#[test]
fn aurora_sbymirbruq() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([183.0, 92.0], [528.0, 706.0]).len, 991.404);
}
#[test]
fn aurora_linugahtrw() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([2.0, 609.0], [957.0, 460.0]).len, 991.871);
}
#[test]
fn aurora_ypnvqodpgr() {
    let aurora = aurora_mesh();
    known_failure!(aurora.path([1016.0, 581.0], [121.0, 327.0]).len, 983.865);
}
#[test]
fn aurora_sloarfedam() {
    let aurora = aurora_mesh();
    known_failure!(aurora.path([945.0, 569.0], [245.0, 26.0]).len, 984.542);
}
#[test]
fn aurora_zahcctcson() {
    let aurora = aurora_mesh();
    known_failure!(aurora.path([744.0, 175.0], [58.0, 628.0]).len, 1000.59);
}
#[test]
fn aurora_utzbaerofi() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([89.0, 481.0], [1009.0, 431.0]).len, 996.728);
}
#[test]
fn aurora_ajxgpdryhx() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([262.0, 726.0], [710.0, 212.0]).len, 991.659);
}
#[test]
fn aurora_qrfjvyrgsj() {
    let aurora = aurora_mesh();
    known_failure!(aurora.path([277.0, 728.0], [713.0, 176.0]).len, 1000.7);
}
#[test]
fn aurora_mplapdhkyx() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([510.0, 46.0], [532.0, 699.0]).len, 995.914);
}
#[test]
fn aurora_mequqewfki() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([233.0, 198.0], [919.0, 657.0]).len, 999.093);
}
#[test]
fn aurora_kphmsrgnzu() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([737.0, 232.0], [351.0, 639.0]).len, 988.227);
}
#[test]
fn aurora_bgoftyecoe() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([367.0, 704.0], [782.0, 155.0]).len, 998.706);
}
#[test]
fn aurora_lhyubnnftl() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([688.0, 210.0], [82.0, 687.0]).len, 995.845);
}
#[test]
fn aurora_ncthpmtfwa() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([996.0, 423.0], [79.0, 573.0]).len, 999.01);
}
#[test]
fn aurora_xysskcekzr() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([373.0, 685.0], [715.0, 140.0]).len, 998.232);
}
#[test]
fn aurora_wvwemdxtlj() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([816.0, 120.0], [381.0, 702.0]).len, 1000.6705);
}
#[test]
fn aurora_emppewiede() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([298.0, 48.0], [776.0, 678.0]).len, 996.3369);
}
#[test]
fn aurora_zdpgsmcjvf() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([1008.0, 404.0], [116.0, 714.0]).len, 1009.6711);
}
#[test]
fn aurora_qpebnkmvby() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([87.0, 444.0], [955.0, 439.0]).len, 993.6116);
}
#[test]
fn aurora_cpfpthivun() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([133.0, 671.0], [901.0, 276.0]).len, 991.90805);
}
#[test]
fn aurora_qdwrneyvoc() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([997.0, 402.0], [63.0, 406.0]).len, 994.78382);
}
#[test]
fn aurora_uucrrrxyxe() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([103.0, 554.0], [802.0, 214.0]).len, 997.61857);
}
#[test]
fn aurora_wxaxrrkhhf() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([777.0, 244.0], [192.0, 666.0]).len, 999.93201);
}
#[test]
fn aurora_onlpdumhzo() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([564.0, 30.0], [3.0, 603.0]).len, 994.40793);
}
#[test]
fn aurora_qopihpykql() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([154.0, 93.0], [733.0, 621.0]).len, 993.39839);
}
#[test]
fn aurora_smyiednvkw() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([696.0, 144.0], [1.0, 609.0]).len, 997.58055);
}
#[test]
fn aurora_bftmfyuktv() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([968.0, 355.0], [39.0, 511.0]).len, 1007.6115);
}
#[test]
fn aurora_iezfgesqgx() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([791.0, 96.0], [116.0, 609.0]).len, 1000.729);
}
#[test]
fn aurora_axaqiygsfg() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([78.0, 541.0], [981.0, 422.0]).len, 1000.9445);
}
#[test]
fn aurora_evvflpbxnh() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([842.0, 117.0], [242.0, 613.0]).len, 1002.2897);
}
#[test]
fn aurora_enyzivenpn() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([731.0, 707.0], [77.0, 178.0]).len, 994.75795);
}
#[test]
fn aurora_elxsfhxbrd() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([730.0, 44.0], [215.0, 683.0]).len, 1005.657);
}
#[test]
fn aurora_prodywiphp() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([371.0, 125.0], [596.0, 719.0]).len, 1006.3103);
}
#[test]
fn aurora_kczhyucezn() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([3.0, 161.0], [882.0, 621.0]).len, 996.874);
}
#[test]
fn aurora_jtlzmvsjpq() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([688.0, 68.0], [34.0, 653.0]).len, 1003.2607);
}
#[test]
fn aurora_atbffxxlog() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([33.0, 193.0], [938.0, 538.0]).len, 997.98276);
}
#[test]
fn aurora_xayjbgcbyc() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([990.0, 335.0], [16.0, 173.0]).len, 1008.9775);
}
#[test]
fn aurora_rznxdqthns() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([1021.0, 302.0], [37.0, 198.0]).len, 1014.8737);
}
#[test]
fn aurora_xztopxuipu() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([724.0, 47.0], [519.0, 699.0]).len, 1002.3511);
}
#[test]
fn aurora_gogfzbgliu() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([350.0, 737.0], [708.0, 181.0]).len, 998.59063);
}
#[test]
fn aurora_thkvqbnjjd() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([954.0, 572.0], [14.0, 330.0]).len, 1002.4995);
}
#[test]
fn aurora_chxbrarifj() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([427.0, 688.0], [495.0, 37.0]).len, 1018.9221);
}
#[test]
fn aurora_dzybcewmyz() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([1006.0, 336.0], [126.0, 715.0]).len, 1024.5258);
}
#[test]
fn aurora_zlrncwpefc() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([839.0, 96.0], [103.0, 513.0]).len, 1002.4129);
}
#[test]
fn aurora_hnifvxqham() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([738.0, 167.0], [56.0, 668.0]).len, 1021.4663);
}
#[test]
fn aurora_vbcicspwha() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([997.0, 301.0], [17.0, 180.0]).len, 1016.2511);
}
#[test]
fn aurora_dqeyichdiu() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([154.0, 142.0], [860.0, 637.0]).len, 999.89383);
}
#[test]
fn aurora_vyvheouwnu() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([601.0, 41.0], [742.0, 678.0]).len, 1012.7972);
}
#[test]
fn aurora_gwvayujwmf() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([768.0, 588.0], [142.0, 109.0]).len, 999.31555);
}
#[test]
fn aurora_srupmthfqx() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([363.0, 662.0], [736.0, 120.0]).len, 1006.4688);
}
#[test]
fn aurora_pwcojuhzih() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([901.0, 655.0], [208.0, 222.0]).len, 1002.1547);
}
#[test]
fn aurora_qgodttzzkk() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([959.0, 286.0], [101.0, 552.0]).len, 1003.2709);
}
#[test]
fn aurora_jutfdmnshi() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([592.0, 685.0], [523.0, 28.0]).len, 1009.886);
}
#[test]
fn aurora_ozqcejqrgx() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([392.0, 692.0], [620.0, 54.0]).len, 1012.9085);
}
#[test]
fn aurora_oseuoehmfj() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([743.0, 675.0], [58.0, 152.0]).len, 1009.1445);
}
#[test]
fn aurora_juremniwgt() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([318.0, 659.0], [789.0, 146.0]).len, 1020.0086);
}
#[test]
fn aurora_axlvrsyins() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([813.0, 204.0], [114.0, 605.0]).len, 1006.599);
}
#[test]
fn aurora_gpecbbvcec() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([284.0, 698.0], [785.0, 220.0]).len, 1010.17);
}
#[test]
fn aurora_czmrtbkeaw() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([991.0, 325.0], [12.0, 442.0]).len, 1024.4687);
}
#[test]
fn aurora_cabmdwikjf() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([49.0, 582.0], [753.0, 109.0]).len, 1007.5878);
}
#[test]
fn aurora_bqkblytwrk() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([276.0, 29.0], [589.0, 676.0]).len, 1009.2622);
}
#[test]
fn aurora_jmnztmwwcn() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([981.0, 397.0], [30.0, 425.0]).len, 1018.7957);
}
#[test]
fn aurora_rfbslayvlr() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([204.0, 150.0], [951.0, 615.0]).len, 1017.1941);
}
#[test]
fn aurora_cuemoawpkd() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([381.0, 669.0], [602.0, 36.0]).len, 1015.8513);
}
#[test]
fn aurora_ulmkbbetbo() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([732.0, 264.0], [63.0, 623.0]).len, 1011.1207);
}
#[test]
fn aurora_hfopunbdku() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([287.0, 38.0], [657.0, 689.0]).len, 1020.4903);
}
#[test]
fn aurora_yanetscsaw() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([116.0, 615.0], [962.0, 325.0]).len, 1010.8377);
}
#[test]
fn aurora_dbcbkjozut() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([79.0, 682.0], [705.0, 101.0]).len, 1014.833);
}
#[test]
fn aurora_mwgwmzvmmv() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([880.0, 580.0], [62.0, 183.0]).len, 1004.4071);
}
#[test]
fn aurora_zrzxvmunrw() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([281.0, 93.0], [604.0, 715.0]).len, 1016.5928);
}
#[test]
fn aurora_epdfwpjjxl() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([997.0, 420.0], [89.0, 659.0]).len, 1028.8545);
}
#[test]
fn aurora_fwumqhdqfy() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([876.0, 166.0], [117.0, 554.0]).len, 1014.7948);
}
#[test]
fn aurora_rbcbkidrsw() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([735.0, 254.0], [90.0, 662.0]).len, 1018.8879);
}
#[test]
fn aurora_zgsfpzzpps() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([726.0, 92.0], [171.0, 695.0]).len, 1013.5684);
}
#[test]
fn aurora_oxsexuynni() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([884.0, 142.0], [284.0, 583.0]).len, 1027.3267);
}
#[test]
fn aurora_snpilvedpa() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([328.0, 667.0], [788.0, 158.0]).len, 1030.0145);
}
#[test]
fn aurora_ghqnjszyst() {
    let aurora = aurora_mesh();
    known_failure!(aurora.path([981.0, 575.0], [237.0, 65.0]).len, 1012.0565);
}
#[test]
fn aurora_tdydurltqb() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([884.0, 131.0], [371.0, 653.0]).len, 1021.6322);
}
#[test]
fn aurora_yphvhcsztv() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([392.0, 686.0], [708.0, 108.0]).len, 1018.3937);
}
#[test]
fn aurora_shmqznfuoc() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([5.0, 526.0], [947.0, 358.0]).len, 1028.2151);
}
#[test]
fn aurora_barvvfmlhn() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([162.0, 621.0], [822.0, 128.0]).len, 1018.9767);
}
#[test]
fn aurora_kvonvwdwfr() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([242.0, 90.0], [338.0, 748.0]).len, 1018.9908);
}
#[test]
fn aurora_nmkyqityei() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([627.0, 738.0], [120.0, 171.0]).len, 1026.9651);
}
#[test]
fn aurora_bqwzfafaee() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([980.0, 408.0], [1.0, 376.0]).len, 1028.7121);
}
#[test]
fn aurora_xrdhnftxdo() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([130.0, 598.0], [980.0, 292.0]).len, 1015.4153);
}
#[test]
fn aurora_odmrgstxyo() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([829.0, 179.0], [162.0, 636.0]).len, 1029.1984);
}
#[test]
fn aurora_ekjelhsauq() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([686.0, 705.0], [538.0, 32.0]).len, 1023.8319);
}
#[test]
fn aurora_cgxondifjb() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([833.0, 158.0], [158.0, 645.0]).len, 1034.0333);
}
#[test]
fn aurora_kqxmvmsxig() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([109.0, 174.0], [783.0, 708.0]).len, 1017.7859);
}
#[test]
fn aurora_edwaypebuh() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([165.0, 134.0], [983.0, 561.0]).len, 1018.7645);
}
#[test]
fn aurora_tjlbsswhgd() {
    let aurora = aurora_mesh();
    known_failure!(aurora.path([932.0, 606.0], [188.0, 122.0]).len, 1021.7925);
}
#[test]
fn aurora_mrzkdcmjza() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([622.0, 734.0], [648.0, 61.0]).len, 1032.5187);
}
#[test]
fn aurora_uifbmmdmcg() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([67.0, 682.0], [741.0, 156.0]).len, 1034.5237);
}
#[test]
fn aurora_vusdxxxnba() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([541.0, 720.0], [739.0, 46.0]).len, 1028.0938);
}
#[test]
fn aurora_jrlsavuylv() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([1000.0, 305.0], [130.0, 633.0]).len, 1042.2494);
}
#[test]
fn aurora_fynzeeuhjz() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([341.0, 748.0], [378.0, 132.0]).len, 1031.5485);
}
#[test]
fn aurora_ivguinjtbg() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([274.0, 746.0], [700.0, 264.0]).len, 1026.2939);
}
#[test]
fn aurora_yuqffhkdct() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([886.0, 138.0], [375.0, 724.0]).len, 1020.6364);
}
#[test]
fn aurora_vryjdnrbta() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([944.0, 423.0], [42.0, 498.0]).len, 1029.9101);
}
#[test]
fn aurora_gnrlstmcfg() {
    let aurora = aurora_mesh();
    known_failure!(aurora.path([874.0, 687.0], [254.0, 63.0]).len, 1024.2445);
}
#[test]
fn aurora_pokablutef() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([874.0, 378.0], [24.0, 678.0]).len, 1025.7853);
}
#[test]
fn aurora_valfurpzre() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([799.0, 196.0], [272.0, 677.0]).len, 1028.992);
}
#[test]
fn aurora_kxieopvstp() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([372.0, 666.0], [796.0, 90.0]).len, 1035.3108);
}
#[test]
fn aurora_yoheofksfc() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([511.0, 43.0], [635.0, 715.0]).len, 1036.4987);
}
#[test]
fn aurora_tyxplximxl() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([285.0, 702.0], [758.0, 85.0]).len, 1025.3019);
}
#[test]
fn aurora_vivfnatbah() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([53.0, 200.0], [970.0, 633.0]).len, 1018.7662);
}
#[test]
fn aurora_vhrewswebe() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([470.0, 52.0], [385.0, 719.0]).len, 1039.9285);
}
#[test]
fn aurora_rgyqkjhejb() {
    let aurora = aurora_mesh();
    known_failure!(aurora.path([946.0, 635.0], [222.0, 123.0]).len, 1028.4326);
}
#[test]
fn aurora_pctnscvrvq() {
    let aurora = aurora_mesh();
    known_failure!(aurora.path([942.0, 564.0], [147.0, 144.0]).len, 1029.0294);
}
#[test]
fn aurora_dthbuxghgy() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([635.0, 739.0], [155.0, 120.0]).len, 1033.738);
}
#[test]
fn aurora_hfkknxhpkx() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([629.0, 736.0], [186.0, 110.0]).len, 1040.1032);
}
#[test]
fn aurora_atpeksecww() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([76.0, 663.0], [759.0, 130.0]).len, 1038.0811);
}
#[test]
fn aurora_gwdqtbcrup() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([625.0, 728.0], [81.0, 147.0]).len, 1036.1352);
}
#[test]
fn aurora_lvmxzqodrp() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([580.0, 685.0], [181.0, 79.0]).len, 1038.3029);
}
#[test]
fn aurora_noruiaolxx() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([913.0, 533.0], [106.0, 123.0]).len, 1030.9363);
}
#[test]
fn aurora_arjwntinue() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([71.0, 132.0], [621.0, 724.0]).len, 1036.9646);
}
#[test]
fn aurora_thismgrrct() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([49.0, 510.0], [1020.0, 312.0]).len, 1042.9785);
}
#[test]
fn aurora_yeigiykobe() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([800.0, 196.0], [119.0, 657.0]).len, 1042.7508);
}
#[test]
fn aurora_qwlnvxopit() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([751.0, 691.0], [74.0, 187.0]).len, 1028.1199);
}
#[test]
fn aurora_ieztesoozb() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([914.0, 248.0], [151.0, 699.0]).len, 1025.585);
}
#[test]
fn aurora_bzxopdispa() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([817.0, 245.0], [189.0, 695.0]).len, 1033.2575);
}
#[test]
fn aurora_nmzjojfmam() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([999.0, 304.0], [26.0, 521.0]).len, 1047.6106);
}
#[test]
fn aurora_cfdfznscio() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([995.0, 341.0], [7.0, 483.0]).len, 1042.689);
}
#[test]
fn aurora_sajzlcthcr() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([593.0, 721.0], [192.0, 131.0]).len, 1039.5355);
}
#[test]
fn aurora_dhnqqzster() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([189.0, 114.0], [736.0, 685.0]).len, 1037.6433);
}
#[test]
fn aurora_iqpwhpegff() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([75.0, 686.0], [752.0, 175.0]).len, 1049.59);
}
#[test]
fn aurora_tnbnaufyop() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([742.0, 126.0], [181.0, 727.0]).len, 1040.2008);
}
#[test]
fn aurora_elnhgrdjmp() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([135.0, 680.0], [794.0, 190.0]).len, 1043.9505);
}
#[test]
fn aurora_ewlrbxwtpe() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([630.0, 60.0], [398.0, 707.0]).len, 1043.8218);
}
#[test]
fn aurora_ubulloixrp() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([814.0, 181.0], [127.0, 638.0]).len, 1041.7375);
}
#[test]
fn aurora_hathhacvjb() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([704.0, 66.0], [0.0, 614.0]).len, 1029.4557);
}
#[test]
fn aurora_opabxwmxzo() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([714.0, 230.0], [18.0, 669.0]).len, 1038.0273);
}
#[test]
fn aurora_wlkgkudvub() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([429.0, 703.0], [521.0, 32.0]).len, 1045.7003);
}
#[test]
fn aurora_yedoiiayab() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([856.0, 156.0], [228.0, 642.0]).len, 1057.4308);
}
#[test]
fn aurora_cayrtuyytr() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([899.0, 159.0], [109.0, 544.0]).len, 1041.8984);
}
#[test]
fn aurora_jjbahtawfg() {
    let aurora = aurora_mesh();
    known_failure!(aurora.path([916.0, 561.0], [142.0, 97.0]).len, 1033.6165);
}
#[test]
fn aurora_fyqpzillnc() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([189.0, 137.0], [953.0, 623.0]).len, 1040.3236);
}
#[test]
fn aurora_taramoqzlk() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([154.0, 149.0], [924.0, 615.0]).len, 1038.5857);
}
#[test]
fn aurora_tbmnwkgtvd() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([304.0, 654.0], [808.0, 120.0]).len, 1039.4372);
}
#[test]
fn aurora_lxkfzhcaip() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([396.0, 110.0], [335.0, 738.0]).len, 1046.1984);
}
#[test]
fn aurora_ohrtrvgzqn() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([763.0, 279.0], [53.0, 592.0]).len, 1036.1649);
}
#[test]
fn aurora_lqpfgzwpon() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([40.0, 649.0], [748.0, 98.0]).len, 1043.5326);
}
#[test]
fn aurora_yjswofvffk() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([115.0, 114.0], [671.0, 718.0]).len, 1045.7259);
}
#[test]
fn aurora_iuumziiqny() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([801.0, 699.0], [102.0, 134.0]).len, 1037.3738);
}
#[test]
fn aurora_myrcughvhi() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([148.0, 668.0], [972.0, 294.0]).len, 1050.5965);
}
#[test]
fn aurora_upwvbmmzpk() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([870.0, 125.0], [129.0, 590.0]).len, 1051.3635);
}
#[test]
fn aurora_msblcpecaq() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([0.0, 473.0], [905.0, 171.0]).len, 1048.11);
}
#[test]
fn aurora_gdbawzmtsn() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([812.0, 271.0], [118.0, 649.0]).len, 1044.3065);
}
#[test]
fn aurora_ypkoorkomf() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([34.0, 460.0], [1008.0, 345.0]).len, 1055.8405);
}
#[test]
fn aurora_xeusaifqvl() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([113.0, 644.0], [790.0, 76.0]).len, 1040.8908);
}
#[test]
fn aurora_nwtqayqdon() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([655.0, 720.0], [261.0, 29.0]).len, 1048.328);
}
#[test]
fn aurora_sbpybkdkyl() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([395.0, 627.0], [791.0, 62.0]).len, 1045.6776);
}
#[test]
fn aurora_kixbjafziz() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([214.0, 642.0], [865.0, 147.0]).len, 1058.5512);
}
#[test]
fn aurora_tcjjkkokfe() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([89.0, 561.0], [886.0, 172.0]).len, 1046.5966);
}
#[test]
fn aurora_qqzraxohem() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([117.0, 147.0], [856.0, 694.0]).len, 1044.3686);
}
#[test]
fn aurora_zfogyyeylc() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([220.0, 73.0], [902.0, 655.0]).len, 1046.2102);
}
#[test]
fn aurora_enjxxpsedx() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([609.0, 715.0], [236.0, 83.0]).len, 1049.9614);
}
#[test]
fn aurora_mfnvomjatr() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([608.0, 711.0], [83.0, 137.0]).len, 1047.461);
}
#[test]
fn aurora_jnkkbxzuff() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([71.0, 530.0], [1015.0, 318.0]).len, 1054.8325);
}
#[test]
fn aurora_mulqtmubfq() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([112.0, 173.0], [934.0, 561.0]).len, 1051.85);
}
#[test]
fn aurora_jnpmkgyspy() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([1002.0, 425.0], [74.0, 688.0]).len, 1057.3649);
}
#[test]
fn aurora_ccsjrcjoeq() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([361.0, 735.0], [465.0, 63.0]).len, 1057.2282);
}
#[test]
fn aurora_vgglfhagxc() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([1014.0, 306.0], [116.0, 674.0]).len, 1062.6103);
}
#[test]
fn aurora_oiztvembox() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([588.0, 721.0], [659.0, 66.0]).len, 1050.7028);
}
#[test]
fn aurora_rihacydqnl() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([809.0, 102.0], [58.0, 567.0]).len, 1050.5933);
}
#[test]
fn aurora_wplqwymxtw() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([68.0, 550.0], [899.0, 162.0]).len, 1059.9533);
}
#[test]
fn aurora_zxvbokmbjo() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([857.0, 690.0], [76.0, 160.0]).len, 1046.0208);
}
#[test]
fn aurora_xafceqhvuz() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([836.0, 250.0], [136.0, 703.0]).len, 1046.3291);
}
#[test]
fn aurora_iprdvalmqg() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([736.0, 123.0], [14.0, 655.0]).len, 1051.9877);
}
#[test]
fn aurora_lwltmkatma() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([32.0, 165.0], [928.0, 650.0]).len, 1042.3754);
}
#[test]
fn aurora_itlvpajcwo() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([329.0, 743.0], [414.0, 79.0]).len, 1059.1344);
}
#[test]
fn aurora_aqgseghwpv() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([869.0, 235.0], [16.0, 639.0]).len, 1049.7446);
}
#[test]
fn aurora_hmhfazaajh() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([841.0, 144.0], [127.0, 658.0]).len, 1073.0762);
}
#[test]
fn aurora_iqoxaqwfdh() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([148.0, 563.0], [940.0, 153.0]).len, 1047.8525);
}
#[test]
fn aurora_wrwtzexirf() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([179.0, 130.0], [970.0, 606.0]).len, 1052.5179);
}
#[test]
fn aurora_dvuorlrrdi() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([172.0, 697.0], [770.0, 238.0]).len, 1052.5848);
}
#[test]
fn aurora_lsxgvgcyax() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([48.0, 664.0], [927.0, 329.0]).len, 1057.0439);
}
#[test]
fn aurora_pgljincpvv() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([677.0, 714.0], [555.0, 19.0]).len, 1057.1375);
}
#[test]
fn aurora_wheuhptjhf() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([130.0, 656.0], [820.0, 88.0]).len, 1060.0573);
}
#[test]
fn aurora_mxapihrluw() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([838.0, 99.0], [83.0, 561.0]).len, 1053.9903);
}
#[test]
fn aurora_mnutchljrb() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([384.0, 699.0], [547.0, 27.0]).len, 1057.9248);
}
#[test]
fn aurora_fhfjavvtea() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([796.0, 98.0], [309.0, 695.0]).len, 1056.4009);
}
#[test]
fn aurora_ninjrcider() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([940.0, 433.0], [15.0, 167.0]).len, 1066.2878);
}
#[test]
fn aurora_kfbgnopfln() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([0.0, 607.0], [873.0, 235.0]).len, 1054.2336);
}
#[test]
fn aurora_ynxdgwjtuv() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([753.0, 76.0], [132.0, 724.0]).len, 1055.0198);
}
#[test]
fn aurora_lxvjztipfs() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([872.0, 136.0], [229.0, 635.0]).len, 1071.2331);
}
#[test]
fn aurora_ojxwwraspa() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([363.0, 102.0], [335.0, 745.0]).len, 1068.0746);
}
#[test]
fn aurora_qiofcmceqj() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([960.0, 587.0], [11.0, 168.0]).len, 1052.2824);
}
#[test]
fn aurora_wfvaemhiyk() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([776.0, 198.0], [141.0, 727.0]).len, 1063.5421);
}
#[test]
fn aurora_oltqmplhdm() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([582.0, 34.0], [638.0, 703.0]).len, 1063.2197);
}
#[test]
fn aurora_pnupvcjtif() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([354.0, 707.0], [908.0, 139.0]).len, 1058.5008);
}
#[test]
fn aurora_ogdtlapskf() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([1017.0, 433.0], [34.0, 424.0]).len, 1064.2754);
}
#[test]
fn aurora_ecknqdzvgn() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([347.0, 731.0], [168.0, 97.0]).len, 1060.2302);
}
#[test]
fn aurora_rcntlgrouv() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([725.0, 278.0], [14.0, 642.0]).len, 1057.5821);
}
#[test]
fn aurora_yxqzfjvmjx() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([752.0, 138.0], [1.0, 631.0]).len, 1063.6049);
}
#[test]
fn aurora_uioausdzoz() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([8.0, 581.0], [881.0, 133.0]).len, 1071.9477);
}
#[test]
fn aurora_uxaxfqcdtb() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([821.0, 147.0], [280.0, 724.0]).len, 1062.9317);
}
#[test]
fn aurora_bhwutlbnly() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([746.0, 248.0], [288.0, 733.0]).len, 1061.3282);
}
#[test]
fn aurora_zzkiisyqxf() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([762.0, 253.0], [111.0, 706.0]).len, 1061.167);
}
#[test]
fn aurora_efnoesutxk() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([756.0, 227.0], [14.0, 621.0]).len, 1061.1829);
}
#[test]
fn aurora_jykctquick() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([318.0, 722.0], [861.0, 125.0]).len, 1068.3091);
}
#[test]
fn aurora_hpqwmtalmf() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([69.0, 650.0], [942.0, 268.0]).len, 1060.3116);
}
#[test]
fn aurora_yrbsueafgo() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([964.0, 435.0], [49.0, 451.0]).len, 1069.0434);
}
#[test]
fn aurora_jnaqruekbr() {
    let aurora = aurora_mesh();
    known_failure!(aurora.path([939.0, 571.0], [103.0, 174.0]).len, 1070.1654);
}
#[test]
fn aurora_rvkcepsviv() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([832.0, 205.0], [141.0, 726.0]).len, 1071.6588);
}
#[test]
fn aurora_wbeysmrhak() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([76.0, 130.0], [927.0, 564.0]).len, 1062.6823);
}
#[test]
fn aurora_mazefrtipu() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([926.0, 256.0], [37.0, 615.0]).len, 1057.5907);
}
#[test]
fn aurora_tpdfudsefj() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([780.0, 725.0], [99.0, 202.0]).len, 1063.6161);
}
#[test]
fn aurora_itnnzbnary() {
    let aurora = aurora_mesh();
    known_failure!(aurora.path([980.0, 588.0], [155.0, 165.0]).len, 1064.7545);
}
#[test]
fn aurora_wdvychvmfz() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([18.0, 587.0], [984.0, 439.0]).len, 1067.1807);
}
#[test]
fn aurora_xjwpqtjofi() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([40.0, 627.0], [756.0, 60.0]).len, 1060.2935);
}
#[test]
fn aurora_gmshxavcka() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([80.0, 687.0], [769.0, 120.0]).len, 1076.2361);
}
#[test]
fn aurora_iroibvtntn() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([23.0, 657.0], [931.0, 354.0]).len, 1071.8706);
}
#[test]
fn aurora_ccjqfuljqs() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([337.0, 746.0], [160.0, 103.0]).len, 1066.6171);
}
#[test]
fn aurora_bxatvmgsxj() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([808.0, 118.0], [335.0, 666.0]).len, 1069.7688);
}
#[test]
fn aurora_wphixcmxji() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([897.0, 654.0], [114.0, 192.0]).len, 1063.7989);
}
#[test]
fn aurora_etmfeuxvdb() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([34.0, 677.0], [1020.0, 583.0]).len, 1082.4165);
}
#[test]
fn aurora_jgreumqxlo() {
    let aurora = aurora_mesh();
    known_failure!(aurora.path([987.0, 623.0], [204.0, 100.0]).len, 1067.4429);
}
#[test]
fn aurora_yfggbrbces() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([751.0, 78.0], [96.0, 692.0]).len, 1068.0877);
}
#[test]
fn aurora_jufrafdujr() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([65.0, 141.0], [900.0, 674.0]).len, 1063.7702);
}
#[test]
fn aurora_lgzcjguojo() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([366.0, 727.0], [684.0, 122.0]).len, 1074.1009);
}
#[test]
fn aurora_xxflnkyqio() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([57.0, 448.0], [1002.0, 428.0]).len, 1077.3664);
}
#[test]
fn aurora_dnspgdiylq() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([725.0, 48.0], [9.0, 662.0]).len, 1071.0622);
}
#[test]
fn aurora_yuhcfdjegt() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([71.0, 170.0], [930.0, 619.0]).len, 1061.8972);
}
#[test]
fn aurora_pwzodplmad() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([866.0, 116.0], [155.0, 654.0]).len, 1085.53);
}
#[test]
fn aurora_suacixanvu() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([52.0, 639.0], [800.0, 183.0]).len, 1076.9483);
}
#[test]
fn aurora_eaggzuyhcl() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([123.0, 162.0], [958.0, 582.0]).len, 1074.6262);
}
#[test]
fn aurora_gtvyqbvblr() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([67.0, 690.0], [788.0, 158.0]).len, 1089.307);
}
#[test]
fn aurora_oqpmobvjlw() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([246.0, 722.0], [794.0, 88.0]).len, 1072.6185);
}
#[test]
fn aurora_hujjdsrdpb() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([29.0, 605.0], [781.0, 83.0]).len, 1068.2169);
}
#[test]
fn aurora_eksjsgfwjg() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([758.0, 277.0], [182.0, 721.0]).len, 1075.1892);
}
#[test]
fn aurora_cvxigsdobj() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([136.0, 105.0], [772.0, 680.0]).len, 1073.0619);
}
#[test]
fn aurora_fwonxuegki() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([3.0, 644.0], [745.0, 241.0]).len, 1070.4251);
}
#[test]
fn aurora_vbhdagcxae() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([57.0, 642.0], [813.0, 191.0]).len, 1085.155);
}
#[test]
fn aurora_sbzitzeklk() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([826.0, 84.0], [151.0, 681.0]).len, 1074.3721);
}
#[test]
fn aurora_olwtxyoflu() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([232.0, 685.0], [852.0, 133.0]).len, 1089.0184);
}
#[test]
fn aurora_tzeaqjicbo() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([789.0, 218.0], [155.0, 731.0]).len, 1079.1427);
}
#[test]
fn aurora_viaojylgvu() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([47.0, 685.0], [993.0, 341.0]).len, 1095.0758);
}
#[test]
fn aurora_bthojcgvmt() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([179.0, 117.0], [1000.0, 598.0]).len, 1074.3074);
}
#[test]
fn aurora_ojlpbqgraq() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([116.0, 705.0], [847.0, 195.0]).len, 1074.189);
}
#[test]
fn aurora_oqhotfgqhb() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([864.0, 108.0], [238.0, 662.0]).len, 1088.5511);
}
#[test]
fn aurora_yzoilvnaze() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([36.0, 646.0], [802.0, 140.0]).len, 1088.3798);
}
#[test]
fn aurora_gkgpooltxv() {
    let aurora = aurora_mesh();
    known_failure!(aurora.path([989.0, 598.0], [152.0, 168.0]).len, 1079.792);
}
#[test]
fn aurora_gtxutwnuie() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([311.0, 699.0], [819.0, 102.0]).len, 1083.4738);
}
#[test]
fn aurora_pdwwodvnaw() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([881.0, 230.0], [0.0, 641.0]).len, 1074.0278);
}
#[test]
fn aurora_tlllqqafpu() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([961.0, 447.0], [55.0, 453.0]).len, 1085.2016);
}
#[test]
fn aurora_zvmrkbqnxa() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([158.0, 125.0], [952.0, 644.0]).len, 1086.9142);
}
#[test]
fn aurora_doawwtyjqa() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([846.0, 105.0], [65.0, 566.0]).len, 1080.5805);
}
#[test]
fn aurora_wnjukyvkgy() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([812.0, 91.0], [152.0, 701.0]).len, 1077.9138);
}
#[test]
fn aurora_vpcmbduhwa() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([159.0, 705.0], [814.0, 177.0]).len, 1085.3328);
}
#[test]
fn aurora_exvjpgqajr() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([110.0, 121.0], [738.0, 684.0]).len, 1075.9822);
}
#[test]
fn aurora_yrpdvsqvzz() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([788.0, 159.0], [82.0, 703.0]).len, 1098.8684);
}
#[test]
fn aurora_sxwtilcdgg() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([72.0, 664.0], [804.0, 216.0]).len, 1086.3925);
}
#[test]
fn aurora_vglswnrsms() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([78.0, 609.0], [847.0, 116.0]).len, 1088.4016);
}
#[test]
fn aurora_xoejbbyohd() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([250.0, 684.0], [941.0, 151.0]).len, 1087.6429);
}
#[test]
fn aurora_aychuxfomv() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([569.0, 24.0], [334.0, 715.0]).len, 1092.3505);
}
#[test]
fn aurora_okckfnwamb() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([826.0, 90.0], [276.0, 695.0]).len, 1084.4033);
}
#[test]
fn aurora_voumiyqkrd() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([333.0, 755.0], [739.0, 116.0]).len, 1079.5976);
}
#[test]
fn aurora_qqunawcxyu() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([78.0, 689.0], [784.0, 196.0]).len, 1091.8364);
}
#[test]
fn aurora_ohkmpajzxk() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([130.0, 717.0], [781.0, 59.0]).len, 1082.2023);
}
#[test]
fn aurora_uxoyzcgunx() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([380.0, 78.0], [327.0, 750.0]).len, 1090.6357);
}
#[test]
fn aurora_ujzoasrini() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([769.0, 69.0], [3.0, 594.0]).len, 1081.7998);
}
#[test]
fn aurora_zrmmhyuxuo() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([3.0, 659.0], [925.0, 344.0]).len, 1085.8277);
}
#[test]
fn aurora_ckcaigrfzn() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([930.0, 307.0], [7.0, 635.0]).len, 1080.6315);
}
#[test]
fn aurora_wiydcxclle() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([851.0, 157.0], [139.0, 692.0]).len, 1101.7203);
}
#[test]
fn aurora_mexjhtdbdl() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([644.0, 148.0], [304.0, 748.0]).len, 1093.6667);
}
#[test]
fn aurora_kdriotyfap() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([137.0, 701.0], [841.0, 148.0]).len, 1100.8814);
}
#[test]
fn aurora_srbhvdpkyh() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([9.0, 518.0], [935.0, 150.0]).len, 1093.32);
}
#[test]
fn aurora_rrcrtavacn() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([17.0, 661.0], [777.0, 210.0]).len, 1091.5112);
}
#[test]
fn aurora_iqhpnorraa() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([1021.0, 350.0], [49.0, 682.0]).len, 1105.1879);
}
#[test]
fn aurora_fgcpsghygo() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([56.0, 673.0], [1007.0, 321.0]).len, 1105.7451);
}
#[test]
fn aurora_fkbqdcqpvk() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([840.0, 182.0], [113.0, 717.0]).len, 1095.2473);
}
#[test]
fn aurora_dxbrabawda() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([83.0, 697.0], [957.0, 303.0]).len, 1108.7885);
}
#[test]
fn aurora_ftgaalboab() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([68.0, 661.0], [821.0, 174.0]).len, 1104.4252);
}
#[test]
fn aurora_bgkadwuifw() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([48.0, 643.0], [798.0, 207.0]).len, 1093.8601);
}
#[test]
fn aurora_lbsyvomghi() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([357.0, 735.0], [752.0, 73.0]).len, 1090.8151);
}
#[test]
fn aurora_xditioolwy() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([0.0, 617.0], [754.0, 279.0]).len, 1085.8279);
}
#[test]
fn aurora_plthxodmew() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([783.0, 186.0], [59.0, 695.0]).len, 1099.823);
}
#[test]
fn aurora_zacuskuzds() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([352.0, 735.0], [631.0, 56.0]).len, 1099.2037);
}
#[test]
fn aurora_awrydbssur() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([819.0, 79.0], [89.0, 650.0]).len, 1091.1437);
}
#[test]
fn aurora_eimdnrhbos() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([76.0, 620.0], [863.0, 141.0]).len, 1101.2165);
}
#[test]
fn aurora_eqgrulngdm() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([903.0, 155.0], [184.0, 718.0]).len, 1106.5873);
}
#[test]
fn aurora_aqeszwzfjx() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([812.0, 95.0], [150.0, 721.0]).len, 1097.1694);
}
#[test]
fn aurora_xsevaqwfio() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([1009.0, 402.0], [0.0, 635.0]).len, 1096.4528);
}
#[test]
fn aurora_mvpsgopqdk() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([305.0, 719.0], [777.0, 100.0]).len, 1097.7203);
}
#[test]
fn aurora_ehfvclspzf() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([84.0, 621.0], [901.0, 160.0]).len, 1093.2823);
}
#[test]
fn aurora_hrbfeujvxv() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([128.0, 178.0], [960.0, 640.0]).len, 1085.7275);
}
#[test]
fn aurora_btfgunhhsf() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([100.0, 158.0], [943.0, 596.0]).len, 1086.1153);
}
#[test]
fn aurora_xnoksxdnaf() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([205.0, 60.0], [599.0, 713.0]).len, 1096.3202);
}
#[test]
fn aurora_edobjqxgsj() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([874.0, 170.0], [40.0, 586.0]).len, 1093.7328);
}
#[test]
fn aurora_cvmxgwgkiy() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([79.0, 685.0], [946.0, 295.0]).len, 1091.2468);
}
#[test]
fn aurora_tgxpjxsulw() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([316.0, 736.0], [815.0, 86.0]).len, 1103.6204);
}
#[test]
fn aurora_syrdgvymwj() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([317.0, 754.0], [657.0, 131.0]).len, 1101.3279);
}
#[test]
fn aurora_klzqoyoahw() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([323.0, 745.0], [653.0, 82.0]).len, 1104.373);
}
#[test]
fn aurora_bzxyidoyml() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([819.0, 253.0], [3.0, 596.0]).len, 1094.1186);
}
#[test]
fn aurora_wybklnfxax() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([762.0, 73.0], [353.0, 742.0]).len, 1096.8566);
}
#[test]
fn aurora_ayudeuwntk() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([788.0, 70.0], [103.0, 699.0]).len, 1097.3119);
}
#[test]
fn aurora_cmlyfgcvox() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([16.0, 627.0], [792.0, 106.0]).len, 1096.4103);
}
#[test]
fn aurora_qkaqyezxzo() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([65.0, 642.0], [987.0, 297.0]).len, 1102.0939);
}
#[test]
fn aurora_pfwpnrjwse() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([9.0, 667.0], [955.0, 437.0]).len, 1104.0966);
}
#[test]
fn aurora_howzynlbxp() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([851.0, 172.0], [53.0, 613.0]).len, 1108.4147);
}
#[test]
fn aurora_vgxhnsqenb() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([40.0, 670.0], [786.0, 275.0]).len, 1110.2192);
}
#[test]
fn aurora_wklbetpdvo() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([775.0, 717.0], [177.0, 75.0]).len, 1100.4302);
}
#[test]
fn aurora_kfpwbmpzzk() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([768.0, 260.0], [1.0, 633.0]).len, 1096.6247);
}
#[test]
fn aurora_tissavcqrf() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([151.0, 733.0], [822.0, 168.0]).len, 1114.6805);
}
#[test]
fn aurora_sfkiuornnp() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([518.0, 35.0], [362.0, 723.0]).len, 1111.2067);
}
#[test]
fn aurora_poxlclocah() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([49.0, 665.0], [811.0, 107.0]).len, 1107.5912);
}
#[test]
fn aurora_wkudhcsxae() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([599.0, 717.0], [579.0, 34.0]).len, 1106.6263);
}
#[test]
fn aurora_xvpbszpmmz() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([870.0, 160.0], [268.0, 689.0]).len, 1115.7576);
}
#[test]
fn aurora_vytqcllrjf() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([263.0, 728.0], [812.0, 79.0]).len, 1104.9207);
}
#[test]
fn aurora_shtdvftiul() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([853.0, 187.0], [37.0, 658.0]).len, 1100.2835);
}
#[test]
fn aurora_xkdibyfoxo() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([1023.0, 414.0], [3.0, 582.0]).len, 1114.0306);
}
#[test]
fn aurora_wklwgviivq() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([966.0, 626.0], [122.0, 160.0]).len, 1097.3395);
}
#[test]
fn aurora_acclmrimyi() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([884.0, 144.0], [79.0, 619.0]).len, 1118.86);
}
#[test]
fn aurora_yhavgiaxrm() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([47.0, 617.0], [996.0, 293.0]).len, 1109.2477);
}
#[test]
fn aurora_awsexwenrs() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([226.0, 720.0], [850.0, 142.0]).len, 1118.3891);
}
#[test]
fn aurora_vvlscybcvq() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([92.0, 709.0], [853.0, 250.0]).len, 1103.0609);
}
#[test]
fn aurora_birgzgamcv() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([987.0, 618.0], [41.0, 147.0]).len, 1096.2784);
}
#[test]
fn aurora_wyuplffwar() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([776.0, 255.0], [5.0, 644.0]).len, 1102.6386);
}
#[test]
fn aurora_jpvaqozngo() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([869.0, 132.0], [329.0, 672.0]).len, 1116.5478);
}
#[test]
fn aurora_koknwlrrhw() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([940.0, 150.0], [153.0, 653.0]).len, 1107.1249);
}
#[test]
fn aurora_eerbeihkgf() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([821.0, 194.0], [89.0, 688.0]).len, 1112.4352);
}
#[test]
fn aurora_dpmxfcmxfp() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([75.0, 144.0], [971.0, 589.0]).len, 1103.8122);
}
#[test]
fn aurora_vfxvljxyir() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([609.0, 730.0], [555.0, 17.0]).len, 1117.4709);
}
#[test]
fn aurora_awduroujhi() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([16.0, 611.0], [819.0, 173.0]).len, 1116.5963);
}
#[test]
fn aurora_hdnguicdhq() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([96.0, 176.0], [952.0, 623.0]).len, 1104.2441);
}
#[test]
fn aurora_dvkmhdnhgv() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([26.0, 649.0], [787.0, 276.0]).len, 1110.3193);
}
#[test]
fn aurora_ikfqpcppgh() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([810.0, 138.0], [12.0, 644.0]).len, 1116.0613);
}
#[test]
fn aurora_tnmptlbtte() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([887.0, 119.0], [238.0, 684.0]).len, 1122.6222);
}
#[test]
fn aurora_vpsowcnnkw() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([835.0, 179.0], [145.0, 726.0]).len, 1123.3291);
}
#[test]
fn aurora_zuvfleazek() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([117.0, 670.0], [872.0, 112.0]).len, 1129.7237);
}
#[test]
fn aurora_znplmpusuo() {
    let aurora = aurora_mesh();
    known_failure!(aurora.path([854.0, 679.0], [144.0, 107.0]).len, 1108.788);
}
#[test]
fn aurora_qirlykrube() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([779.0, 285.0], [49.0, 690.0]).len, 1114.7555);
}
#[test]
fn aurora_llafgphpve() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([50.0, 636.0], [850.0, 123.0]).len, 1126.6631);
}
#[test]
fn aurora_hcvmmangig() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([34.0, 672.0], [957.0, 276.0]).len, 1111.8828);
}
#[test]
fn aurora_hwqhrwqenu() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([62.0, 683.0], [858.0, 178.0]).len, 1114.6736);
}
#[test]
fn aurora_ghjbfevjal() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([895.0, 134.0], [293.0, 677.0]).len, 1123.073);
}
#[test]
fn aurora_elfcxklckn() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([830.0, 156.0], [77.0, 689.0]).len, 1131.2928);
}
#[test]
fn aurora_owevyjjvha() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([809.0, 176.0], [41.0, 680.0]).len, 1122.342);
}
#[test]
fn aurora_emgadfptfd() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([98.0, 191.0], [934.0, 652.0]).len, 1104.4646);
}
#[test]
fn aurora_rtjwphpfmc() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([793.0, 78.0], [89.0, 684.0]).len, 1114.1801);
}
#[test]
fn aurora_casjdjtwew() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([331.0, 671.0], [866.0, 103.0]).len, 1122.9406);
}
#[test]
fn aurora_zqtajdupdn() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([52.0, 614.0], [874.0, 160.0]).len, 1129.4693);
}
#[test]
fn aurora_spbyjrgbdq() {
    let aurora = aurora_mesh();
    known_failure!(aurora.path([979.0, 590.0], [98.0, 191.0]).len, 1120.2311);
}
#[test]
fn aurora_ztyphjfxsp() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([104.0, 166.0], [961.0, 633.0]).len, 1110.02);
}
#[test]
fn aurora_oakxxstbyn() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([875.0, 691.0], [121.0, 107.0]).len, 1114.0472);
}
#[test]
fn aurora_alguihfbwg() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([67.0, 569.0], [904.0, 144.0]).len, 1127.4946);
}
#[test]
fn aurora_cmlaknrbnv() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([843.0, 156.0], [59.0, 675.0]).len, 1131.6711);
}
#[test]
fn aurora_hrokohqkcv() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([284.0, 674.0], [896.0, 119.0]).len, 1125.2731);
}
#[test]
fn aurora_tjjxwfyogr() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([991.0, 589.0], [61.0, 140.0]).len, 1113.5999);
}
#[test]
fn aurora_kgubohftar() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([808.0, 207.0], [10.0, 627.0]).len, 1112.4008);
}
#[test]
fn aurora_oxtnchorbl() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([855.0, 144.0], [27.0, 600.0]).len, 1127.1473);
}
#[test]
fn aurora_pwdqwakdtl() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([596.0, 722.0], [565.0, 34.0]).len, 1123.7803);
}
#[test]
fn aurora_yyeggxbmdi() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([182.0, 723.0], [847.0, 148.0]).len, 1137.4998);
}
#[test]
fn aurora_wtlgktygaa() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([145.0, 97.0], [969.0, 616.0]).len, 1115.7011);
}
#[test]
fn aurora_jasjmgcbzu() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([718.0, 682.0], [134.0, 115.0]).len, 1119.0245);
}
#[test]
fn aurora_sgztwbnkjs() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([856.0, 192.0], [5.0, 643.0]).len, 1118.2746);
}
#[test]
fn aurora_guooaikkit() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([1.0, 576.0], [928.0, 151.0]).len, 1132.1721);
}
#[test]
fn aurora_vgpuexkkct() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([950.0, 605.0], [94.0, 119.0]).len, 1113.1997);
}
#[test]
fn aurora_pplmauwqup() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([891.0, 163.0], [54.0, 645.0]).len, 1121.9482);
}
#[test]
fn aurora_vydupdxvld() {
    let aurora = aurora_mesh();
    assert_delta!(aurora.path([993.0, 290.0], [34.0, 622.0]).len, 1123.2226);
}
