from flask import Flask, render_template
from dotenv import load_dotenv

from routes.ShoesRoutes import shoes

config = load_dotenv()

app = Flask(__name__)

@app.route('/')
def index():
  return render_template('index.html')

app.register_blueprint(shoes, url_prefix='/shoes')

if __name__ == '__main__':
  app.run(debug=True)