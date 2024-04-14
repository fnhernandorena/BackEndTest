import { Module } from '@nestjs/common';
import { ShoesController } from './shoes.controller';
import { ShoesService } from './shoes.service';
import { MongooseModule } from '@nestjs/mongoose';
import { ShoeSchema } from './schemas/shoe.schema';

@Module({
  imports: [MongooseModule.forFeature([{name: 'Shoe', schema: ShoeSchema}])],
  controllers: [ShoesController],
  providers: [ShoesService]
})
export class ShoesModule {}
