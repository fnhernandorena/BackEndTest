import { Body, Controller, Delete, Get, NotFoundException, Param, Post, Put } from '@nestjs/common';
import { ShoesService } from './shoes.service';
import { Shoe } from './schemas/shoe.schema';

@Controller('shoes')
export class ShoesController {
    constructor (private shoeService: ShoesService){}

    @Get()
    async getAllShoes(): Promise<Shoe[]>{
        return await this.shoeService.findAllShoes()
    }

    @Get(':id')
    async getShoe(
        @Param('id')
        id: string
    ): Promise<Shoe>{
        const book = await this.shoeService.findShoe(id)
     if (!book){ throw new NotFoundException('Shoe not found')}
     return book
    }

    @Post()
    async createShoe(
        @Body() shoe: Shoe
    ): Promise<Shoe> {
       return this.shoeService.createShoe(shoe)
    }

    @Put(':id')
    async updateShoe(
        @Param('id')
        id: string,
        @Body() shoe: Shoe
    ): Promise<Shoe>{
        const book = await this.shoeService.updateShoe(id, shoe)
     if (!book){ throw new NotFoundException('Shoe not found')}
     return book
    }

    @Delete(':id')
    async deleteShoe(
        @Param('id')
        id: string
    ): Promise<Shoe>{
        const book = await this.shoeService.deleteShoe(id)
        if (!book){ throw new NotFoundException('Shoe not found')}
        return book 
    }
}
