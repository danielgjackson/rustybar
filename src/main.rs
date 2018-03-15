use std::env;

const DEFAULT_INVERT: bool = true;
const DEFAULT_NARROW: bool = true;
const DEFAULT_HEIGHT: u32 = 4;

const CODE128: [u32; 108] = [
  //BSBSBSbs val  CA  CB  CC = representation
  0x21222200, //   0  SP  SP  00 = 32
  0x22212200, //   1   !   !  01 = 33
  0x22222100, //   2   "   "  02 = 34
  0x12122300, //   3   #   #  03 = 35
  0x12132200, //   4   $   $  04 = 36
  0x13122200, //   5   %   %  05 = 37
  0x12221300, //   6   &   &  06 = 38
  0x12231200, //   7   '   '  07 = 39
  0x13221200, //   8   (   (  08 = 40
  0x22121300, //   9   )   )  09 = 41
  0x22131200, //  10   *   *  10 = 42
  0x23121200, //  11   +   +  11 = 43
  0x11223200, //  12   ,   ,  12 = 44
  0x12213200, //  13   -   -  13 = 45
  0x12223100, //  14   .   .  14 = 46
  0x11322200, //  15   /   /  15 = 47
  0x12312200, //  16   0   0  16 = 48
  0x12322100, //  17   1   1  17 = 49
  0x22321100, //  18   2   2  18 = 50
  0x22113200, //  19   3   3  19 = 51
  0x22123100, //  20   4   4  20 = 52
  0x21321200, //  21   5   5  21 = 53
  0x22311200, //  22   6   6  22 = 54
  0x31213100, //  23   7   7  23 = 55
  0x31122200, //  24   8   8  24 = 56
  0x32112200, //  25   9   9  25 = 57
  0x32122100, //  26   :   :  26 = 58
  0x31221200, //  27   ;   ;  27 = 59
  0x32211200, //  28   <   <  28 = 60
  0x32221100, //  29   =   =  29 = 61
  0x21212300, //  30   >   >  30 = 62
  0x21232100, //  31   ?   ?  31 = 63
  0x23212100, //  32   @   @  32 = 64
  0x11132300, //  33   A   A  33 = 65
  0x13112300, //  34   B   B  34 = 66
  0x13132100, //  35   C   C  35 = 67
  0x11231300, //  36   D   D  36 = 68
  0x13211300, //  37   E   E  37 = 69
  0x13231100, //  38   F   F  38 = 70
  0x21131300, //  39   G   G  39 = 71
  0x23111300, //  40   H   H  40 = 72
  0x23131100, //  41   I   I  41 = 73
  0x11213300, //  42   J   J  42 = 74
  0x11233100, //  43   K   K  43 = 75
  0x13213100, //  44   L   L  44 = 76
  0x11312300, //  45   M   M  45 = 77
  0x11332100, //  46   N   N  46 = 78
  0x13312100, //  47   O   O  47 = 79
  0x31312100, //  48   P   P  48 = 80
  0x21133100, //  49   Q   Q  49 = 81
  0x23113100, //  50   R   R  50 = 82
  0x21311300, //  51   S   S  51 = 83
  0x21331100, //  52   T   T  52 = 84
  0x21313100, //  53   U   U  53 = 85
  0x31112300, //  54   V   V  54 = 86
  0x31132100, //  55   W   W  55 = 87
  0x33112100, //  56   X   X  56 = 88
  0x31211300, //  57   Y   Y  57 = 89
  0x31231100, //  58   Z   Z  58 = 90
  0x33211100, //  59   [   [  59 = 91
  0x31411100, //  60   \   \  60 = 92
  0x22141100, //  61   ]   ]  61 = 93
  0x43111100, //  62   ^   ^  62 = 94
  0x11122400, //  63   _   _  63 = 95
  0x11142200, //  64 NUL   '  64 = 96
  0x12112400, //  65 SOH   a  65 = 97
  0x12142100, //  66 STX   b  66 = 98
  0x14112200, //  67 ETX   c  67 = 99
  0x14122100, //  68 EOT   d  68 =100
  0x11221400, //  69 ENQ   e  69 =101
  0x11241200, //  70 ACK   f  70 =102
  0x12211400, //  71 BEL   g  71 =103
  0x12241100, //  72  BS   h  72 =104
  0x14211200, //  73  HT   i  73 =105
  0x14221100, //  74  LF   j  74 =106
  0x24121100, //  75  VT   k  75 =107
  0x22111400, //  76  FF   l  76 =108
  0x41311100, //  77  CR   m  77 =109
  0x24111200, //  78  SO   n  78 =110
  0x13411100, //  79  SI   o  79 =111
  0x11124200, //  80 DLE   p  80 =112
  0x12114200, //  81 DC1   q  81 =113
  0x12124100, //  82 DC2   r  82 =114
  0x11421200, //  83 DC3   s  83 =115
  0x12411200, //  84 DC4   t  84 =116
  0x12421100, //  85 NAK   u  85 =117
  0x41121200, //  86 SYN   v  86 =118
  0x42111200, //  87 ETB   w  87 =119
  0x42121100, //  88 CAN   x  88 =120
  0x21214100, //  89  EM   y  89 =121
  0x21412100, //  90 SUB   z  90 =122
  0x41212100, //  91 ESC   {  91 =123
  0x11114300, //  92  FS   |  92 =124
  0x11134100, //  93  GS   }  93 =125
  0x13114100, //  94  RS   ~  94 =126
  0x11411300, //  95  US DEL  95 =127
  0x11431100, //  96 _F3 _F3  96 =128 _F3="FNC 3"
  0x41111300, //  97 _F2 _F2  97 =129 _F2="FNC 2"
  0x41131100, //  98 _SH _SH  98 =130 _SH="SHIFT"
  0x11314100, //  99 _CC _CC  99 =131 _CC="CODE C"
  0x11413100, // 100 _CB _F4 _CB =132 _CB="CODE B" _F4="FNC 4"
  0x31114100, // 101 _F4 _CA _CA =133 _F4="FNC 4" _CA="CODE A"
  0x41113100, // 102 _F1 _F1 _F1 =134 _F1="FNC 1"
  0x21141200, // 103 _SA _SA _SA =135 _SA="START (Code A)"
  0x21121400, // 104 _SB _SB _SB =136 _SB="START (Code B)"
  0x21123200, // 105 _SC _SC _SC =137 _SC="START (Code C)"
  0x23311120, // 106 _ST _ST _ST =138 _ST="STOP"
  0x0000000A, // 107 _QZ _QZ _QZ =139 _QZ="QUIET"
];

fn generate(bytes: &Vec<u8>) -> Vec<bool> {
    let mut out: Vec<bool> = vec!();
    for b in bytes {
        let code = CODE128[*b as usize];
        for i in 0..4 {
            let shift = (3 - i) * 8;

            let band_mask = 0x000000F0u32 << shift;
            let band = (code & band_mask) >> (shift + 4);
            for _j in 0..band {
                out.push(true);
            }

            let space_mask = 0x0000000Fu32 << shift;
            let space = (code & space_mask) >> shift;
            for _j in 0..space {
                out.push(false);
            }
        }
    }
    return out;
}

fn render_wide(input: &Vec<bool>, invert: bool) -> Vec<char> {
    let mut out: Vec<char> = vec!();
    for i in 0..input.len() {
        let a = input[i] ^ invert;
        if a {
            out.push('█');
        } else {
            out.push(' ');
        }
    }
    return out;
}

fn render_narrow(input: &Vec<bool>, invert: bool) -> Vec<char> {
    let mut out: Vec<char> = vec!();
    let out_width = (input.len() + 1) / 2;
    for i in 0..out_width {
        let a = input[i * 2] ^ invert;
        let b;
        if i * 2 + 1 >= input.len() {
            b = false;
        } else {
            b = input[i * 2 + 1] ^ invert;
        }
        if a && b {
            out.push('█');
        } else if a && !b {
            out.push('\u{258C}');   // left
        } else if !a && b {
            out.push('\u{2590}');   // right
        } else if !a && !b {
            out.push(' ');
        }
    }
    return out;
}

fn main() {
    let height = DEFAULT_HEIGHT;
    let invert = DEFAULT_INVERT;
    let narrow = DEFAULT_NARROW;
    let args: Vec<String> = env::args().collect();
    let mut bytes: Vec<u8> = vec!();    
    bytes.push(104);    // start (B)
    for arg in &args[1..] {
        for c in arg.chars() { 
            bytes.push(c as u8 - 32);
        }
        // let val: u8 = arg.parse().unwrap();
        // bytes.push(val);
    }

    // checksum = SUM<(i+1) * X[i]> % 103
    let mut checksum: u32 = 0;
    let mut position: u32 = 0;
    for b in &bytes {
        if position == 0 {
            checksum = *b as u32;
        } else {
            checksum += position * *b as u32;
        }
        position += 1;
    }
    checksum %= 103;
    bytes.push(checksum as u8); // checksum
    bytes.push(106);            // stop

    // Add quiet zone (only after checksum calculation)
    bytes.insert(0, 107);       // quiet
    bytes.push(107);            // quiet

    let bars = generate(&bytes);

    let out;
    if narrow {
        out = render_narrow(&bars, invert);
    } else {
        out = render_wide(&bars, invert);
    }
    
    let out_str: String = out.into_iter().collect();
    for _j in 0..height {
        println!("{}", out_str);
    }

}
