import mongoose from "mongoose";

const shoeSchema = new mongoose.Schema({
    brand:{
        type:String,
        required:true,
        trim:true
    },
    model:{
        type:String,
        required:true,
        trim:true
    },
    size:{
        type:Number,
        required:true
    }
})

export default mongoose.model('Shoe', shoeSchema);