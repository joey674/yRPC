/*
    非对称加密:RSA 
    生成一对密钥：公钥（public key）和私钥（private key),公钥用于加密,私钥用于解密,且这个关系是单向的,即用公钥加密的信息只能用对应的私钥解密
    所以公钥可以分发给任何人,私钥只能由生成者持有,所以如果想给某人发消息, 用对方展示的公钥加密即可, 只有对方用自己的私钥才能解密
    明文的e次方对n取模得到密文, 密文的d次方对n取模得到明文(其中 由明文是没法通过公钥逆推秘文获得的)
*/
use num_bigint::{BigUint,RandBigInt};
use num_traits::{One,Zero};
use num_integer::Integer;
use rand::rngs::OsRng;
use core::convert::TryInto;


/*  
    n (BigUint): 这是公钥的模数（modulus），它是两个大质数的乘积𝑝×𝑞; 模数是公开信息
    e (BigUint): 这是公钥的指数（exponent），通常是一个较小的质数 这个指数也是公开信息
*/
#[derive(Debug)]
pub struct PublicKey {
    n: BigUint,
    e: BigUint,
}
impl PublicKey {
    pub fn encrypt(&self, message: &[u8]) -> Result<Vec<u8>, String> {
        let plaintext = BigUint::from_bytes_be(message);
        if plaintext >= self.n {
            return Err("message too long,message should shorter than key".to_string());
        }
        let cyphertext = plaintext.modpow(&self.e, &self.n);
        Ok(cyphertext.to_bytes_be())
    }
}
/* 
    d (BigUint): 这是私钥的指数（private exponent）用于解密数据,是私密的
    n (BigUint): 这是公钥的模数（modulus），与私钥的模数相同,是公开的
*/
#[derive(Debug)]
pub struct PrivateKey {
    pub(crate) n : BigUint,
    pub(crate) d: BigUint,
}
impl PrivateKey {
    pub fn decrypt(&self, message: &[u8]) -> Result<Vec<u8>, String> {
        let cyphertext = BigUint::from_bytes_be(message);
        let plaintext = cyphertext.modpow(&self.d, &self.n);
        Ok(plaintext.to_bytes_be())
    }
}
/* 
    暂时限制bit位数为256 这样测试的时候pq可以用u128来测试
*/
pub fn generate_keys(bit_size: usize) -> (PublicKey, PrivateKey) {
    let mut rng = OsRng;
    let e = BigUint::from(65537u32);  

    let p = generate_prime(&mut rng, bit_size / 2);// 因为n=p*q,所以让这两个质数的位数等于目标位数的一半
    let q = generate_prime(&mut rng, bit_size / 2);
    // println!("p: {:?}, q: {:?}", p, q);
    let n = &p * &q;
    let phi = (&p - 1u32) * (&q - 1u32);
    let d = e.modinv(&phi).unwrap();

    (
        PublicKey { 
            n: n.clone(), 
            e: e.clone() 
        },
        PrivateKey {
            n,
            d,
        },
    )

}

fn generate_prime(rng: &mut OsRng, bits: usize) -> BigUint {
    loop {
        let prime = rng.gen_biguint(bits.try_into().unwrap());
        if is_prime(&prime) {
            return prime;
        }
    }
}

/*  
    Miller-Rabin 质数测试：
    这个算法是一个概率性算法，通过选择随机的 'witnesses' 来测试数的质数性
    参数 k 控制测试的轮次，增加 k 可以减少错误判断一个合数为质数的概率。
*/
fn is_prime (num: &BigUint) -> bool {
    mrtest(num)
}

fn qmul(a: &BigUint, b: &BigUint, m: &BigUint) -> BigUint {
    let a = a % m;
    let b = b % m;
    let mut result = BigUint::zero();
    let mut a_temp = a.clone();
    let mut b_temp = b.clone();

    while b_temp > BigUint::zero() {
        if &b_temp & BigUint::one() == BigUint::one() {
            result = (result + &a_temp) % m;
        }
        a_temp = (&a_temp * &BigUint::from(2u32)) % m;
        b_temp >>= 1;
    }
    result
}

fn qpow(mut a: BigUint, mut n: BigUint, m: &BigUint) -> BigUint {
    let mut res = BigUint::one();
    a = a % m;

    while n > BigUint::zero() {
        if &n & BigUint::one() == BigUint::one() {
            res = qmul(&res, &a, m);
        }
        a = qmul(&a, &a, m);
        n >>= 1;
    }
    res
}

fn mrtest(n: &BigUint) -> bool {
    if n < &BigUint::from(3u32) || n.is_even() {
        return n == &BigUint::from(2u32);
    }

    let mut u = n - 1u32;
    let mut t = 0;
    while u.is_even() {
        u >>= 1;
        t += 1;
    }

    let bases = vec![
        BigUint::from(2u32),
        BigUint::from(325u32),
        BigUint::from(9375u32),
        BigUint::from(28178u32),
        BigUint::from(450775u32),
        BigUint::from(9780504u32),
        BigUint::from(1795265022u32),
    ];

    for a in bases {
        let mut v = qpow(a, u.clone(), n);
        if v == BigUint::one() || v == n - 1u32 || v == BigUint::zero() {// v == 2
            continue;
        }
        let mut j = 1;
        while j <= t {
            v = qmul(&v, &v, n);
            if v == n - 1u32 && j != t {
                v = BigUint::one();
                break;
            }
            if v == BigUint::one() {
                return false;
            }
            j += 1;
        }
        if v != BigUint::one() {
            return false;
        }
    }
    true
}


#[test]
fn test_generate_keys() {
    let key = generate_keys(256);
    println!("private key: {:?}", key);
}

#[test]
fn test_is_prime() {
    let prime = 190908181517808712773339059221652727331 as u128;
    let prime = BigUint::from(prime);
    assert_eq!(is_prime(&prime), true);
}