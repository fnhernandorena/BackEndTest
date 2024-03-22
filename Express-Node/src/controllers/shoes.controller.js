import Shoe from '../models/shoe.model.js'

export const getShoes = async (req, res) => {
    const shoes = await Shoe.find();
    res.json(shoes)
};

export const createShoe = async (req, res) => {

    const { brand, model, size } = req.body;

    const newShoe = new Shoe({
        brand,
        model,
        size
    });

    const savedShoe = await newShoe.save();
    res.json(savedShoe);
};
export const getShoe = async (req, res) => {
    const shoe = await Shoe.findById(req.params.id);
    if (!shoe) return res.status(404).json({ message: 'Shoe not found' });
    res.json(shoe);
};
export const updateShoe = async (req, res) => {
    const shoe = await Shoe.findByIdAndUpdate(req.params.id, req.body, { new: true });
    if (!shoe) return res.status(404).json({ message: 'Shoe not found' });
    res.json(shoe);
};
export const deleteShoe = async (req, res) => {
    const shoe = await Shoe.findByIdAndDelete(req.params.id);
    if (!shoe) return res.status(402).json({ message: 'Shoe not found' });
    return res.sendStatus(204);
};