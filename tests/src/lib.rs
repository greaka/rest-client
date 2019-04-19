#[cfg(test)]
mod tests {
    extern crate reqwest;
    extern crate rest_client;
    use rest_client::*;
    use serde::Deserialize;

    #[rest("https://jsonplaceholder.typicode.com/comments?postId={}", vec)]
    #[rest("https://jsonplaceholder.typicode.com/comments/{}")]
    #[derive(Deserialize, Debug)]
    struct Comment {
        #[serde(rename = "postId")]
        post_id: u64,
        id: u64,
        name: String,
        email: String,
        body: String,
    }

    impl std::cmp::PartialEq for Comment {
        fn eq(&self, other: &Comment) -> bool {
            if self.post_id != other.post_id {
                return false;
            }
            if self.id != other.id {
                return false;
            }
            if self.name != other.name {
                return false;
            }
            if self.email != other.email {
                return false;
            }
            if self.body != other.body {
                return false;
            }
            true
        }
    }
    impl std::cmp::Eq for Comment {}

    #[test]
    fn get_single_comment() {
        /*{
            "postId": 1,
            "id": 1,
            "name": "id labore ex et quam laborum",
            "email": "Eliseo@gardner.biz",
            "body": "laudantium enim quasi est quidem magnam voluptate ipsam eos\ntempora quo necessitatibus\ndolor quam autem quasi\nreiciendis et nam sapiente accusantium"
        }*/
        let check = Box::new(Comment {
            post_id: 1,
            id: 1,
            name: "id labore ex et quam laborum".to_owned(),
            email: "Eliseo@gardner.biz".to_owned(),
            body: "laudantium enim quasi est quidem magnam voluptate ipsam eos\ntempora quo necessitatibus\ndolor quam autem quasi\nreiciendis et nam sapiente accusantium".to_owned()
        });

        let comment = Comment::get((1.to_string(),).into()).unwrap();
        assert_eq!(comment, check);
    }

    #[test]
    fn get_multiple_comments() {
        /*[
  {
    "postId": 1,
    "id": 1,
    "name": "id labore ex et quam laborum",
    "email": "Eliseo@gardner.biz",
    "body": "laudantium enim quasi est quidem magnam voluptate ipsam eos\ntempora quo necessitatibus\ndolor quam autem quasi\nreiciendis et nam sapiente accusantium"
  },
  {
    "postId": 1,
    "id": 2,
    "name": "quo vero reiciendis velit similique earum",
    "email": "Jayne_Kuhic@sydney.com",
    "body": "est natus enim nihil est dolore omnis voluptatem numquam\net omnis occaecati quod ullam at\nvoluptatem error expedita pariatur\nnihil sint nostrum voluptatem reiciendis et"
  },
  {
    "postId": 1,
    "id": 3,
    "name": "odio adipisci rerum aut animi",
    "email": "Nikita@garfield.biz",
    "body": "quia molestiae reprehenderit quasi aspernatur\naut expedita occaecati aliquam eveniet laudantium\nomnis quibusdam delectus saepe quia accusamus maiores nam est\ncum et ducimus et vero voluptates excepturi deleniti ratione"
  },
  {
    "postId": 1,
    "id": 4,
    "name": "alias odio sit",
    "email": "Lew@alysha.tv",
    "body": "non et atque\noccaecati deserunt quas accusantium unde odit nobis qui voluptatem\nquia voluptas consequuntur itaque dolor\net qui rerum deleniti ut occaecati"
  },
  {
    "postId": 1,
    "id": 5,
    "name": "vero eaque aliquid doloribus et culpa",
    "email": "Hayden@althea.biz",
    "body": "harum non quasi et ratione\ntempore iure ex voluptates in ratione\nharum architecto fugit inventore cupiditate\nvoluptates magni quo et"
  }
]*/
        let check1 = Box::new(Comment {
            post_id: 1,
            id: 1,
            name: "id labore ex et quam laborum".to_owned(),
            email: "Eliseo@gardner.biz".to_owned(),
            body: "laudantium enim quasi est quidem magnam voluptate ipsam eos\ntempora quo necessitatibus\ndolor quam autem quasi\nreiciendis et nam sapiente accusantium".to_owned()
        });
        let check2 = Box::new(Comment {
            post_id: 1,
            id: 2,
            name: "quo vero reiciendis velit similique earum".to_owned(),
            email: "Jayne_Kuhic@sydney.com".to_owned(),
            body: "est natus enim nihil est dolore omnis voluptatem numquam\net omnis occaecati quod ullam at\nvoluptatem error expedita pariatur\nnihil sint nostrum voluptatem reiciendis et".to_owned()
        });
        let check3 = Box::new(Comment {
            post_id: 1,
            id: 3,
            name: "odio adipisci rerum aut animi".to_owned(),
            email: "Nikita@garfield.biz".to_owned(),
            body: "quia molestiae reprehenderit quasi aspernatur\naut expedita occaecati aliquam eveniet laudantium\nomnis quibusdam delectus saepe quia accusamus maiores nam est\ncum et ducimus et vero voluptates excepturi deleniti ratione".to_owned()
        });
        let check4 = Box::new(Comment {
            post_id: 1,
            id: 4,
            name: "alias odio sit".to_owned(),
            email: "Lew@alysha.tv".to_owned(),
            body: "non et atque\noccaecati deserunt quas accusantium unde odit nobis qui voluptatem\nquia voluptas consequuntur itaque dolor\net qui rerum deleniti ut occaecati".to_owned()
        });
        let check5 = Box::new(Comment {
            post_id: 1,
            id: 5,
            name: "vero eaque aliquid doloribus et culpa".to_owned(),
            email: "Hayden@althea.biz".to_owned(),
            body: "harum non quasi et ratione\ntempore iure ex voluptates in ratione\nharum architecto fugit inventore cupiditate\nvoluptates magni quo et".to_owned()
        });
        let check = vec![check1, check2, check3, check4, check5];

        let comment = Comment::gets((1.to_string(),).into()).unwrap();
        assert_eq!(comment, check);
    }
}
