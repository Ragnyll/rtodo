# Blueprint with python pseduocode
```python
# decrypt access token: default to decrypting from .password_store dir but allow override
# figure our the user id and cache it
# /users?username=<username
response = json.loads(requests.get('https://gitlab.com/api/v4/users?username=ragnyll', headers={'Authorization': 'access_token ...........'}).text)
user = json.loads(requests.get('https://gitlab.com/api/v4/users/2436873', headers={'Authorization': 'access_token .........'}).text)
#
# get projects belonging to user
#
projects = json.loads(requests.get('https://gitlab.com/api/v4/users/2436873/projects', headers={'Authorization': 'access_token ...........'}).text)
#
# Get all issues assigned to and logged by the user
response = json.loads(requests.get('https://gitlab.com/api/v4/issues?assignee_id=2436873', headers={'PRIVATE-TOKEN': '.........'}).text)
#
response = json.loads(requests.get('https://gitlab.com/api/v4/issues?author_id=2436873', headers={'PRIVATE-TOKEN': '.............'}).text)
#
# Merge on project title to lightweight object
#
# merge to ~/todo.md
```
