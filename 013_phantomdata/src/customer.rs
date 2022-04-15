use std::{
    marker::PhantomData,
    sync::atomic::{AtomicU64, Ordering},
};

static NEXT_ID: AtomicU64 = AtomicU64::new(1);

// 定义用户类型
pub struct Customer<T> {
    id: u64,
    name: String,
    _type: PhantomData<T>,
}

impl<T> Customer<T> {
    pub fn new(name: String) -> Self {
        Self {
            id: NEXT_ID.fetch_add(1, Ordering::Relaxed),
            name,
            _type: PhantomData::default(),
        }
    }
}

// 定义免费的功能接口
pub trait Free {
    fn feature1(&self);
    fn feature2(&self);
}

// 定义付费的功能接口
pub trait Personal: Free {
    fn advance_feature(&self);
}

impl<T> Free for Customer<T> {
    fn feature1(&self) {
        println!("feature 1 for {}", self.name);
    }

    fn feature2(&self) {
        println!("feature 2 for {}", self.name);
    }
}

// 实现免费功能, 这个就是 Customer<T> 中的 T
pub struct FreePlan;
// 实现付费功能, 这个就是 Customer<T> 中的 T
pub struct PersonalPlan(f32);

// 这是由免费用户转到付费用户的方法
impl From<Customer<FreePlan>> for Customer<PersonalPlan> {
    fn from(c: Customer<FreePlan>) -> Self {
        Self::new(c.name)
    }
}

// 订阅成为付费用户
pub fn subscribe(customer: Customer<FreePlan>, payment: f32) -> Customer<PersonalPlan> {
    let _plan = PersonalPlan(payment);
    // 存储 plan 到 DB
    // ...
    customer.into()
}

// 付费用户的高级功能
impl Personal for Customer<PersonalPlan> {
    fn advance_feature(&self) {
        println!("Dear {}(as our valuable customer {}), enjoy this advanced feature!",
                 self.name,
                 self.id
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_customer() {
        // 一开始是个免费用户
        let customer = Customer::<FreePlan>::new("liwei".into());
        // 使用免费功能
        customer.feature1();
        customer.feature2();
        // 用着用着觉得产品不错, 可以转换成付费用户
        let customer = subscribe(customer, 100.0);
        customer.feature1();
        customer.feature2();
        // 付费用户解锁新技能
        customer.advance_feature();
    }
}