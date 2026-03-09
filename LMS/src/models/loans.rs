use uuid::Uuid;
use chrono::NaiveDateTime;

#[derive(Debug)]
#[allow(dead_code)]
pub struct Loan {
    pub loan_id: Uuid,
    pub copy_id: Uuid,
    pub user_id: Uuid,
    pub issue_date: NaiveDateTime,
    pub due_date: NaiveDateTime,
    pub return_date: Option<NaiveDateTime>,
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct CreateLoanRequest {
    pub user_id: Uuid,
    pub copy_id: Uuid,
    pub due_date: NaiveDateTime,
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct SearchLoanRequestAdmin {
    pub loan_id: Uuid
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct SearchLoanRequestUser {
    pub user_id: Uuid
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct SearchLoanResponse {
    pub user_id: Uuid,
    pub loan_id: Uuid,
    pub copy_id: Uuid,
    pub due_date: NaiveDateTime,
    pub issue_date: NaiveDateTime,
    pub return_date: Option<NaiveDateTime>,
}