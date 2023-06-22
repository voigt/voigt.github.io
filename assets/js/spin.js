const host = 'https://cvo.fermyon.app'
var liked_page = false
console.log("Fetching Likes from document...")

var id_likes = oid_likes ? oid_likes.replaceAll("/", "-") : oid_likes

fetch('' + host + '/api/v1/count/' + id_likes)
  .then(response => response.json())
  .then(data => {
    document.querySelectorAll("span[id='" + oid_likes + "']")[0].innerText = data.count;
    var liked = localStorage.getItem(id_likes);
    if (liked) {
      liked_page = true
      document.querySelectorAll("span[id='likes_button_heart']")[0].style.display = ""
      document.querySelectorAll("span[id='likes_button_emtpty_heart']")[0].style.display = "none"
      document.querySelectorAll("span[id='likes_button_text']")[0].innerText = ""
    }
  })
  .catch(error => {
    console.log("Error getting document:", error);
  });


function like_article(id_likes) {
  fetch('' + host + '/api/v1/count/' + id_likes, {
    method: "POST",
  }).then(response => response.json())
    .then(data => {
      liked_page = true
      localStorage.setItem(id_likes, true);
      document.querySelectorAll("span[id='" + oid_likes + "']")[0].innerText = data.count;
      document.querySelectorAll("span[id='likes_button_heart']")[0].style.display = ""
      document.querySelectorAll("span[id='likes_button_emtpty_heart']")[0].style.display = "none"
      document.querySelectorAll("span[id='likes_button_text']")[0].innerText = ""

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
      document.querySelectorAll("span[id='likes_button_heart']")[0].style.display = "none"
      document.querySelectorAll("span[id='likes_button_emtpty_heart']")[0].style.display = ""
      document.querySelectorAll("span[id='likes_button_text']")[0].innerText = "\xa0Like"
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
