using MongoDB.Driver;

namespace C__Asp.Net.Repositories
{
    public class MongoDbRepo
    {
        public MongoClient Client;

        public IMongoDatabase db;

        public MongoDbRepo()
        {
            Client = new MongoClient("mongodb+srv://user:password@cluster0.xcxigqk.mongodb.net/?retryWrites=true&w=majority&appName=Cluster0");

            db = Client.GetDatabase("test");
        }
    }
}
