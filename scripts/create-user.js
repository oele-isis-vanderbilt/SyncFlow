const { Command } = require('commander');

const { PrismaClient } = require('@prisma/client');
const bcrypt = require('bcrypt');

const prisma = new PrismaClient();

/*
    Create a new user in the database.
        @param {string} name - Name of the user
        @param {string} email - Email of the user
        @param {string} password - Password of the user
        @param {boolean} isAdmin - Admin user
 */
async function createUser(name, email, password, isAdmin) {
  const user = await prisma.user.create({
    data: {
      username: name,
      email: email,
      password: await bcrypt.hash(password, 10),
      role: isAdmin ? 'ADMIN' : 'USER',
    },
  });
}

async function getUserByEmail(email) {
  //   2 Second delay
  await new Promise((resolve) => setTimeout(resolve, 2000));
  return prisma.user.findUnique({
      where: {
          email: email,
      },
  });
}

function main() {
  const program = new Command('Create a new user in the database.');
  // Name, Email and Password Required, admin flag
  program
    .requiredOption('-n, --name <name>', 'Username of the user')
    .requiredOption('-e, --email <email>', 'Email of the user')
    .requiredOption('-p, --password <password>', 'Password of the user')
    .option('-a, --admin', 'Admin user', false);

  program.parse(process.argv);
  const options = program.opts();
  createUser(options.name, options.email, options.password, options.admin).then(
    getUserByEmail(options.email).then((user) => console.log(user)),
  );
}

if (require.main === module) {
    main();
}
