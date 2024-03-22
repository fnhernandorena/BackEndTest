import { Router } from "express";
import { createShoe, deleteShoe, getShoe, getShoes, updateShoe } from "../controllers/shoes.controller.js";

const router = Router();

router.get('/shoes', getShoes)
router.post('/shoes', createShoe)
router.get('/shoes/:id', getShoe)
router.put('/shoes/:id', updateShoe)
router.delete('/shoes/:id', deleteShoe)

export default router;