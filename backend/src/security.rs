use argon2::{
    password_hash::{
        rand_core::OsRng,
        PasswordHasher, SaltString
    },
    Argon2, PasswordHash, PasswordVerifier
};

pub fn hash_password(password: &String) -> Result<String, argon2::password_hash::Error> {
    let salt = SaltString::generate(&mut OsRng);

    let argon2 = Argon2::default();

    let password_hash = argon2.hash_password(password.as_bytes(), &salt)?;

    Ok(password_hash.to_string())
}

pub fn verify_password(password: &String, db_password: &String) -> Result<bool, argon2::password_hash::Error> {
    let parsed_hash = PasswordHash::new(db_password)?;

    Ok(Argon2::default().verify_password(password.as_bytes(), &parsed_hash).is_ok())
}