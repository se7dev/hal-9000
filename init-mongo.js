db.createUser(
    {
        user: "test",
        pwd: "asdf",
        roles: [
            {
                role: "readWrite",
                db: "halDB"
            }
        ]
    }
)