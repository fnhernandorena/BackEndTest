using C__Asp.Net.Models;
using MongoDB.Bson;
using MongoDB.Driver;

namespace C__Asp.Net.Repositories
{
    public class ShoesCollection : IShoesCollection
    { 
        internal MongoDbRepo _repository = new MongoDbRepo();
        private IMongoCollection<Shoe> Collection;

        public ShoesCollection() 
        {
            Collection = _repository.db.GetCollection<Shoe>("shoes");
        }
        public async Task CreateShoe(Shoe shoe)
        {
            await Collection.InsertOneAsync(shoe);
        }

        public async Task DeleteShoe(string id)
        {
            var filter = Builders<Shoe>.Filter.Eq(s => s.Id, new ObjectId(id));
            await Collection.DeleteOneAsync(filter);
        }

        public async Task<List<Shoe>> GetAllShoes()
        {
            return await Collection.FindAsync(new BsonDocument()).Result.ToListAsync();
        }

        public async Task<Shoe> GetShoe(string id)
        {
            return await Collection.FindAsync(new BsonDocument { { "_id", new ObjectId(id)} }).Result.FirstAsync();
        }

        public async Task UpdateShoe(Shoe shoe)
        {
            var filter = Builders<Shoe>.Filter.Eq(s => s.Id, shoe.Id);
            await Collection.ReplaceOneAsync(filter, shoe);
        }
    }
}
