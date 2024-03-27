using MongoDB.Bson;
using MongoDB.Bson.Serialization.Attributes;

namespace C__Asp.Net.Models
{
    public class Shoe
    {
        [BsonId]
        public ObjectId Id {  get; set; }
        public string Brand { get; set; }
        public string Model { get; set; }
        public int Size { get; set; }
    }
}
