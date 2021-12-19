import { fetch_users } from '../pkg/kinesis_db.js';

let users: any = fetch_users(
  '/home/edgeking810/Documents/Rust/kinesis-db/data/users.txt',
  'OhgENBmdz38JOCsRYehi53YCLQFbj99x6ua/dxI5kc8='
);
console.log(users);
