import pkg from '../pkg/kinesis_db.js';

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

const { get_mappings_path, delete_ggt_user } = pkg;

console.log(get_mappings_path());

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

let tryDel = delete_ggt_user('', 'test');
console.log(tryDel);

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
