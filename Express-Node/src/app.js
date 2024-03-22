import express from 'express';
import cors from 'cors'

import shoesRoutes from './routes/shoes.routes.js'

const app = express();

app.use(cors({
    origin: 'http://localhost:5173',
    credentials: true
}));
app.use(express.json());

app.use('/api',shoesRoutes);

export default app;