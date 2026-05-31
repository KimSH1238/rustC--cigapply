// 러스트로 피즈버즈 문제 해결
fn main()
{
// 러스트의 반복문
    for i in 1..21
    {
// 조건과 일치하는지 확인하는 조건문
        if i%3==0 && i%5==0
        {
            println!("FizzBuzz");
        }
        else if i%3==0
        {
            println!("Fizz");
        }
        else if i%5==0
        {
            println!("Buzz");
        }
        else
        {
            println!("{}",i);
        }
    }
println!("\n지식은 무기보다 가치가 있다.\n")
}
