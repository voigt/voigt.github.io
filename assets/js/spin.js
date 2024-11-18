const host = 'https://cvo.fermyon.app'
var liked_page = false

var id_likes = oid_likes ? oid_likes.replaceAll("/", "-") : oid_likes

console.log('> ' + host + '/api/v1/count/' + id_likes)
fetch('' + host + '/api/v1/count/' + id_likes)
  .then(response => response.json())
  .then(data => {
    document.querySelectorAll("span[id='" + oid_likes + "']")[0].innerText = data.count;
    var liked = localStorage.getItem(id_likes);
    if (liked) {
      liked_page = true
      document.querySelectorAll("span[id='button_likes_heart']")[0].style.display = ""
      document.querySelectorAll("span[id='button_likes_emtpty_heart']")[0].style.display = "none"
      document.querySelectorAll("span[id='button_likes_text']")[0].innerText = ""
    }
    var likes_nodes = document.querySelectorAll("span[id^='likes_']")
    console.log("> likes_nodes ", likes_nodes)
    for (var i in likes_nodes) {
        var node = likes_nodes[i]
        console.log("> node ", node)
        var id = node.id ? node.id.replaceAll("/", "-") : node.id
        console.log("> node ", node)
        if (id) {
          toggleLoaders(node)
        }
    }
  })
  .catch(error => {
    console.log("Error getting document:", error);
  });


function toggleLoaders(node){
  var classesString = node.className;
  if(classesString == "") return
  var classes = classesString.split(" ");
  for(var i in classes){  
      node.classList.toggle(classes[i])
  }
}

function like_article(id_likes) {
  fetch('' + host + '/api/v1/count/' + id_likes, {
    method: "POST",
  }).then(response => response.json())
    .then(data => {
      liked_page = true
      localStorage.setItem(id_likes, true);
      document.querySelectorAll("span[id='" + oid_likes + "']")[0].innerText = data.count;
      document.querySelectorAll("span[id='button_likes_heart']")[0].style.display = ""
      document.querySelectorAll("span[id='button_likes_emtpty_heart']")[0].style.display = "none"
      document.querySelectorAll("span[id='button_likes_text']")[0].innerText = ""
      var likes_nodes = document.querySelectorAll("span[id^='likes_']")

      for (var i in likes_nodes) {
          var node = likes_nodes[i]
          console.log("> node ", node)
          var id = node.id ? node.id.replaceAll("/", "-") : node.id
          console.log("> node ", node)
          if (id) {
            toggleLoaders(node)
          }
      }
    })
    .catch(error => {
      console.log("Error getting document:", error);
    });
}

function remove_like_article(id_likes) {
  fetch('' + host + '/api/v1/count/' + id_likes, {
    method: "PATCH",
  }).then(response => response.json())
    .then(data => {
      liked_page = false
      localStorage.removeItem(id_likes);
      document.querySelectorAll("span[id='" + oid_likes + "']")[0].innerText = data.count;
      document.querySelectorAll("span[id='button_likes_heart']")[0].style.display = "none"
      document.querySelectorAll("span[id='button_likes_emtpty_heart']")[0].style.display = ""
      document.querySelectorAll("span[id='button_likes_text']")[0].innerText = "\xa0Like"
      var likes_nodes = document.querySelectorAll("span[id^='likes_']")

      for (var i in likes_nodes) {
          var node = likes_nodes[i]
          console.log("> node ", node)
          var id = node.id ? node.id.replaceAll("/", "-") : node.id
          console.log("> node ", node)
          if (id) {
            toggleLoaders(node)
          }
      }
    })
    .catch(error => {
      console.log("Error getting document:", error);
    });
}

function process_article() {
  var id_likes = oid_likes ? oid_likes.replaceAll("/", "-") : oid_likes
  console.log(id_likes)
  if (!liked_page) {
    like_article(id_likes)
  } else {
    remove_like_article(id_likes)
  }
}
