import bcrypt from 'bcrypt';
import pkg, { save_users } from '../pkg/kinesis_db.js';

// import {
//   get_mappings_path,
//   fetch_users,
//   login_user,
//   moggt,
//   login_ggt_user,
//   register_user,
//   delete_ggt_user,
//   save_users,
//   delete_user,
// } from 'kinesis-db/kinesis_db.js';

const { get_mappings_path, fetch_mappings } = pkg;
const currentPath = '/home/edgeking810/Documents/Rust/kinesis-db/api';

// ENCRYPTION KEY ===> OhgENBmdz38JOCsRYehi53YCLQFbj99x6ua/dxI5kc8=

// const lol = async () => {
//   const all_users = await fetch_users(
//     '/home/edgeking810/Documents/Rust/kinesis-db/data/users.txt',
//     'OhgENBmdz38JOCsRYehi53YCLQFbj99x6ua/dxI5kc8='
//   );
//   console.log(all_users);
// };

// lol();

// try {
//   save_users('', 'users.txt', 'OhgENBmdz38JOCsRYehi53YCLQFbj99x6ua/dxI5kc8=');
// } catch (e) {
//   console.log(e);
// }

// let tryDel = delete_user('', 'test');
// console.log(tryDel);

console.log(`${currentPath}/${get_mappings_path()}`);
let savedUsers = save_users('', `${currentPath}/data/pilon.txt`, '');
console.log(savedUsers);

let mappings = fetch_mappings(`${currentPath}/${get_mappings_path()}`, '');
console.log(mappings);

// let users = fetch_users('')

// let regresult = register_user(
//   all_users,
//   'Kishan',
//   'Takoordyal',
//   'EdgeKing810',
//   'kishan@konnect.dev',
//   'Test123*',
//   0
// );
// console.log(regresult);

// let all_users =
//   'Kishan;Takoordyal;EdgeKing810;kishan@konnect.dev;$2b$10$NMlJ/PLLMdUoVRoP/Tvi5eGUBx/OyHrpiJhm8qT53dV0KQk6Xuahe;0';

// const login = login_ggt_user(all_users, 'EdgeKing810', 'Test123*');
// console.log(login);

const hash = async (data, salt) => {
  const hashedPassword = bcrypt.hashSync(data, salt);
  return hashedPassword;
};

const verify = async (data, hash) => {
  if (bcrypt.compareSync(data, hash)) {
    return 1;
  }

  return 0;
};
