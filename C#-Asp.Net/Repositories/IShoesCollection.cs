using C__Asp.Net.Models;

namespace C__Asp.Net.Repositories
{
    public interface IShoesCollection
    {
        Task<List<Shoe>> GetAllShoes();
        Task<Shoe> GetShoe(string id);
        Task CreateShoe(Shoe shoe);
        Task UpdateShoe(Shoe shoe);
        Task DeleteShoe(string id);
    }
}
