import { Injectable } from '@nestjs/common';
import { Shoe } from './schemas/shoe.schema';
import * as mongoose from 'mongoose';
import { InjectModel } from '@nestjs/mongoose';

@Injectable()
export class ShoesService {
    constructor(
        @InjectModel(Shoe.name)
        private shoeModel: mongoose.Model<Shoe>
    ){}

    async findAllShoes(): Promise<Shoe[]>{
        return await this.shoeModel.find();
    }

    async findShoe(id: string): Promise<Shoe>{
        return await this.shoeModel.findById(id);
    }

    async createShoe(shoe: Shoe): Promise<Shoe>{
        return await this.shoeModel.create(shoe);
    }

    async updateShoe(id: string, shoe: Shoe): Promise<Shoe>{
        return await this.shoeModel.findByIdAndUpdate(id, shoe);
    }

    async deleteShoe(id:string): Promise<Shoe>{
        return await this.shoeModel.findByIdAndDelete(id);
    }

}
