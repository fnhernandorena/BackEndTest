import app from './app.js';
import { connectDB } from './db.js';
import dotenv from "dotenv";

dotenv.config();
connectDB();
app.listen(3000);
console.log('listening on 3000');  