// invitation_handler.rs
use actix::{Handler, Message};
use chrono::{Duration, Local};
#[allow(unused_imports)]
use diesel::result::{DatabaseErrorKind, Error::DatabaseError};
use diesel::{self, prelude::*};
use errors::ServiceError;
use models::{DbExecutor, Invitation};
use uuid::Uuid;



#[derive(Debug, Deserialize)]
pub struct CreateInvitation {
    pub email: String,
}

impl Message for CreateInvitation {
    type Result = Result<Invitation,ServiceError>;
}

impl Handler<CreateInvitation> for DbExecutor {
    type Result = Result<Invitation, ServiceError>;

    fn handle(&mut self,msg: CreateInvitation, _: &mut Self::Context) -> Self::Result {
        use schema::invitations::dsl::*;
        let conn : &PgConnection = &self.0.get().unwrap();

        let new_inviation = Invitation {
            id: Uuid::new_v4(),
            email: msg.email.clone(),
            expires_at: Local::now().naive_local()+Duration::hours(24),
        };

        diesel::insert_into(invitations)
            .values(&new_inviation)
            .execute(conn)
            .map_err(|error| {
                println!("{:#?}", error);// for Debugging Purpose
                ServiceError::InternalServerError
            })?;

        let mut items = invitations
            .filter(email.eq(&new_inviation.email))
            .load::<Invitation>(conn)
            .map_err(|_| ServiceError::InternalServerError)?;
        
        Ok(items.pop().unwrap())

    }
}