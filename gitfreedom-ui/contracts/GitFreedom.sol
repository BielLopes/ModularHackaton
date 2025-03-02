//SPDX-License-Identifier: MIT
pragma solidity 0.8.26;
interface IWallet  {
        // Define Gitfreedom structure
    struct RepoGitfreedom {
        // Repository name
        string name;
        // Repository description
        string description;
        // Repository hash
        string hash;
        // Repository colaborators;
        address[] colaborators;
    }

    function listAll() external view returns (address[] memory, RepoGitfreedom[] memory) ;
    function addRepository(string calldata name, string calldata description, string calldata hash) external;
    function getRepository(string calldata name, address owner) external view returns (string memory, string memory, string memory, address[] memory);
    function listOneRepo(address user, uint256 index) external view returns (RepoGitfreedom memory) ;
    function listAllOwners() external view returns (address[] memory) ;
}

contract Gitfreedom is IWallet {


    mapping(address => RepoGitfreedom[]) repos;
    address[] owners;
    uint256 repoCounter;

    function listOneRepo(address user, uint256 index) public view returns (RepoGitfreedom memory) {
        require(index < repos[user].length, "Index out of bounds");
        return repos[user][index];
    }    

    function listAllOwners() external view returns (address[] memory) {
        return owners;
    }

    function addRepository(string calldata name, string calldata description, string calldata hash) external {
        address owner = msg.sender;
        address[] memory collab = new address[](1);
        collab[1]=owner;
        repoCounter++;
        repos[owner].push(RepoGitfreedom(name,description,hash,collab));
    }

    function getRepository(string calldata name, address owner) external view returns (string memory, string memory, string memory, address[] memory){
        RepoGitfreedom[] memory repo = repos[owner];
        for (uint256 i = 0; i < repo.length; ++i) {
            if(keccak256(abi.encodePacked((repo[i]).name)) == keccak256(abi.encodePacked(name))) return ((repo[i]).description, (repo[i]).hash,(repo[i]).name, repo[i].colaborators);
        }
        return ("","","",new address[](0));
    }

// Function to return all repos for all users
    function listAll() public view returns (address[] memory, RepoGitfreedom[] memory) {
        // Create an array to hold the owners
        address[] memory allOwners = owners;

        // Create a 2D array to hold the repository details
        RepoGitfreedom[] memory allRepos = new RepoGitfreedom[](getNumberOfRepositories());

        uint256 counter = 0;

        // Loop through all owners and their repositories
        for (uint256 i = 0; i < allOwners.length; i++) {
            address owner = allOwners[i];
            RepoGitfreedom[] storage ownerRepos = repos[owner];

            // Loop through each repository of the owner
            for (uint256 j = 0; j < ownerRepos.length; j++) {
                RepoGitfreedom memory  repo = ownerRepos[j];
                allRepos[counter] = RepoGitfreedom(repo.name, repo.description, repo.hash, repo.colaborators);
                counter++;
            }
        }

        return (allOwners, allRepos);
    }

    function getNumberOfRepositories() public view returns(uint256){
        return repoCounter;
    }


}
