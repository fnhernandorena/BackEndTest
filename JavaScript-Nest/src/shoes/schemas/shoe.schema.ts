import { Prop, Schema, SchemaFactory } from "@nestjs/mongoose";


@Schema({
    timestamps: true,
})
export class Shoe{
    @Prop()
    brand: string
    @Prop()
    model: string
    @Prop()
    size: number
}

export const ShoeSchema = SchemaFactory.createForClass(Shoe)