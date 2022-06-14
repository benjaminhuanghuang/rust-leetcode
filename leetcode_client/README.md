# Generate solution file for leetcode problem

## Usage
```
  cargo run -- <problem-id>
```

1. get all leetcode problems and save them in json file
```
reqwest::get(LEETCODE_PROBLEMS_API_URL)
```

2. query problem title slug by fronted id in the json.


3. query the problem detail by the title slug
```
{
		"operationName": "questionData",
		"variables": {
			"titleSlug": "%s"
		},
		"query": "query questionData($titleSlug: String!) {
      question(titleSlug: $titleSlug) 
      {
        questionId  
        questionFrontendId 
        title 
        titleSlug 
        difficulty 
        content 
        codeSnippets {      
          lang      
          langSlug   
          code   
        }
      }
    }"
	}
```
