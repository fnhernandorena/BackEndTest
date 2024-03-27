using C__Asp.Net.Models;
using C__Asp.Net.Repositories;
using Microsoft.AspNetCore.Mvc;

namespace C__Asp.Net.Controllers
{
    [Route("[controller]")]
    [ApiController]
    public class ShoesController : Controller
    {
        private IShoesCollection db =new ShoesCollection();

        [HttpGet]
        public async Task<IActionResult> GetAllShoes()
        {
            return Ok(await db.GetAllShoes());
        }

        [HttpGet("{id}")]
        public async Task<IActionResult> GetShoe(string id)
        {
            return Ok(await db.GetShoe(id));
        }

        [HttpPost]
        public async Task<IActionResult> CreateShoe([FromBody] Shoe shoe)
        {
            if (shoe == null)
                return BadRequest();
            if(shoe.Brand == string.Empty)
            {
                ModelState.AddModelError("Brand", "The shoe has a brand");
            }

            await db.CreateShoe(shoe);

            return Created("Created", true);
        }

        [HttpPut("{id}")]
        public async Task<IActionResult> UpdateShoe([FromBody] Shoe shoe, string id)
        {
            if (shoe == null)
                return BadRequest();
            if (shoe.Brand == string.Empty)
            {
                ModelState.AddModelError("Brand", "The shoe has a brand");
            }

            shoe.Id = new MongoDB.Bson.ObjectId(id);
            await db.UpdateShoe(shoe);

            return Created("Created", true);
        }

        [HttpDelete("{id}")]
        public async Task<IActionResult> DeleteShoe(string id)
        {
            await db.DeleteShoe(id);
            return NoContent();
        }
    }
}
